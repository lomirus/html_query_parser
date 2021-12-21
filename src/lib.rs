#[derive(Debug)]
enum Tag {
    /// Like `<div>`, including `<img>`, `<input>`, etc.
    Start(String),
    /// Like `</div>`
    End(String),
    /// Like `<div />`
    Closing(String),
    /// Like `<!doctype html>`
    Document,
    /// Like `<!-- comment -->`
    Comment,
}

impl Tag {
    pub fn from(tag: String) -> Option<Self> {
        if tag.ends_with(" />") {
            Some(Self::Closing(tag[1..tag.len() - 3].to_string()))
        } else if tag.starts_with("</") {
            Some(Self::End(tag[2..tag.len() - 1].to_string()))
        } else if tag.starts_with("<!--") {
            Some(Self::Comment)
        } else if tag.starts_with("<!") {
            Some(Self::Document)
        } else if tag.starts_with("<") {
            Some(Self::Start(tag[1..tag.len() - 1].to_string()))
        } else {
            None
        }
    }
}

#[derive(Debug)]
enum Token {
    Tag(Tag),
    Text(String),
}

pub fn parse(html: &str) {
    let mut tag_char_stack = Vec::<char>::new();
    let mut txt_char_stack = Vec::<char>::new();
    let mut token_stack = Vec::<Token>::new();
    let mut in_tag_brackets = false;
    for ch in html.chars() {
        match ch {
            '<' => {
                in_tag_brackets = true;
                tag_char_stack.push(ch);
                // In case of pushing empty text tokens to the stack
                if txt_char_stack.len() == 0 {
                    continue;
                }
                // Turn the chars in `txt_char_stack` in to `String` 
                // and clean the char stack. 
                let txt_text = String::from_iter(txt_char_stack);
                txt_char_stack = Vec::new();
                // Push the text we just got to token stack.
                token_stack.push(Token::Text(txt_text));
            }
            '>' => {
                in_tag_brackets = false;
                tag_char_stack.push(ch);
                // Turn the chars in `tag_char_stack` in to `String` 
                // and clean the char stack. 
                let tag_text = String::from_iter(tag_char_stack);
                tag_char_stack = Vec::new();
                // Push the tag with the text we just got to token stack.
                let tag = Tag::from(tag_text.clone())
                    .expect(format!("Invalid tag: {}", tag_text).as_str());
                token_stack.push(Token::Tag(tag));
            }
            _ => match in_tag_brackets {
                true => tag_char_stack.push(ch),
                false => txt_char_stack.push(ch),
            },
        }
    }
    println!("{:?}", token_stack);
}

#[cfg(test)]
mod tests {
    #[test]
    fn paired() {
        crate::parse(
            "
            <header></header>
            <div>Hello, world!</div>
            <footer></footer>",
        );
    }
    #[test]
    fn void() {
        crate::parse("<div />");
    }
    #[test]
    fn self_closing() {
        crate::parse("<img>");
    }
    #[test]
    fn html_anatomy() {
        crate::parse(
            "
            <!doctype html>
            <html>
                <head></head>
                <body></body>
            </html>",
        );
    }
    #[test]
    fn comment() {
        crate::parse("<!-- comment -->");
        crate::parse("<!--comment-->");
    }
}
