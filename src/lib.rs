use std::collections::HashMap;

#[derive(Debug)]
enum Tag {
    /// Like `<div>`, including `<img>`, `<input>`, etc.
    Start(String, HashMap<String, String>),
    /// Like `</div>`
    End(String),
    /// Like `<div />`
    Closing(String, HashMap<String, String>),
    /// Like `<!doctype html>`
    Document,
    /// Like `<!-- comment -->`
    Comment,
}

impl Tag {
    fn from(tag: String) -> Option<Self> {
        if tag.ends_with("/>") {
            let tag_name_start = tag[1..tag.len()]
                .chars()
                .position(|x| x != ' ')
                .expect("tag name cannot be all spaces after \"<\"")
                + 1;
            let tag_name_end_option = tag[tag_name_start..tag.len()]
                .chars()
                .position(|x| x == ' ');
            let tag_name_end = match tag_name_end_option {
                Some(end) => end + tag_name_start,
                None => tag.len() - 2,
            };
            let tag_name = tag[tag_name_start..tag_name_end].to_string();
            let attr_str = tag[tag_name_end..tag.len() - 2].trim().to_string();
            Some(Self::Closing(tag_name, parse_attrs(attr_str)))
        } else if tag.starts_with("</") {
            Some(Self::End(tag[2..tag.len() - 1].to_string()))
        } else if tag.starts_with("<!--") {
            Some(Self::Comment)
        } else if tag.starts_with("<!") {
            Some(Self::Document)
        } else if tag.starts_with("<") {
            let tag_name_start = tag[1..tag.len()]
                .chars()
                .position(|x| x != ' ')
                .expect("tag name cannot be all spaces after \"<\"")
                + 1;
            let tag_name_end_option = tag[tag_name_start..tag.len()]
                .chars()
                .position(|x| x == ' ');
            let tag_name_end = match tag_name_end_option {
                Some(end) => end + tag_name_start,
                None => tag.len() - 1,
            };
            let tag_name = tag[tag_name_start..tag_name_end].to_string();
            let attr_str = tag[tag_name_end..tag.len() - 1].trim().to_string();
            Some(Self::Start(tag_name, parse_attrs(attr_str)))
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
    let mut tag_chars_stack = Vec::<char>::new();
    let mut txt_chars_stack = Vec::<char>::new();
    let mut token_stack = Vec::<Token>::new();
    let mut in_tag_brackets = false;
    for ch in html.chars() {
        match ch {
            '<' => {
                in_tag_brackets = true;
                tag_chars_stack.push(ch);
                // In case of pushing empty text tokens to the stack
                if txt_chars_stack.len() == 0 {
                    continue;
                }
                // Turn the chars in `txt_chars_stack` in to `String`
                // and clean the chars stack.
                let txt_text = String::from_iter(txt_chars_stack);
                txt_chars_stack = Vec::new();
                // Push the text we just got to the token stack.
                token_stack.push(Token::Text(txt_text));
            }
            '>' => {
                in_tag_brackets = false;
                tag_chars_stack.push(ch);
                // Turn the chars in `tag_chars_stack` in to `String`
                // and clean the chars stack.
                let tag_text = String::from_iter(tag_chars_stack);
                tag_chars_stack = Vec::new();
                // Push the tag with the text we just got to the token stack.
                let tag = Tag::from(tag_text.clone())
                    .expect(format!("Invalid tag: {}", tag_text).as_str());
                token_stack.push(Token::Tag(tag));
            }
            _ => match in_tag_brackets {
                true => tag_chars_stack.push(ch),
                false => txt_chars_stack.push(ch),
            },
        }
    }
    println!("{:#?}", token_stack);
}

fn parse_attrs(attr_str: String) -> HashMap<String, String> {
    let mut hashmap = HashMap::new();
    hashmap.insert("attr".to_string(), attr_str);
    hashmap
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
        crate::parse("<div/>");
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
    #[test]
    fn attributes() {
        crate::parse("<img src=\"example.png\" alt=example>");
    }
}
