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

/// Let's take `<img src="example.png" alt=image>` for example.
enum AttrPos {
    /// Here including `src`, `alt`
    Key,
    /// Here including `=`
    Equal,
    /// Here including `example.png`, `image`
    Value(Option<char>),
    /// Here including ` `
    Space,
}

fn parse_attrs(attr_str: String) -> HashMap<String, String> {
    let mut chars_stack: Vec<char> = Vec::new();
    let mut key_stack: Vec<String> = Vec::new();
    let mut value_stack: Vec<String> = Vec::new();
    let mut attr_pos = AttrPos::Key;
    for ch in attr_str.chars() {
        match attr_pos {
            AttrPos::Key => match ch {
                '=' => {
                    attr_pos = AttrPos::Equal;
                    let key = String::from_iter(chars_stack);
                    chars_stack = Vec::new();
                    key_stack.push(key)
                }
                ' ' => {
                    attr_pos = AttrPos::Space;
                    let key = String::from_iter(chars_stack);
                    chars_stack = Vec::new();
                    key_stack.push(key);
                    value_stack.push(String::new())
                }
                _ => chars_stack.push(ch),
            },
            AttrPos::Equal => match ch {
                '\'' => attr_pos = AttrPos::Value(Some('\'')),
                '\"' => attr_pos = AttrPos::Value(Some('\"')),
                _ => {
                    attr_pos = AttrPos::Value(None);
                    chars_stack.push(ch)
                }
            },
            AttrPos::Value(delimiter) => match delimiter {
                None => {
                    if ch == ' ' {
                        attr_pos = AttrPos::Space;
                        let value = String::from_iter(chars_stack);
                        chars_stack = Vec::new();
                        value_stack.push(value)
                    } else {
                        chars_stack.push(ch);
                    }
                }
                Some(quote) => {
                    if ch == quote {
                        let last_char = chars_stack
                            .last()
                            .expect("cannot accesss the last char in `chars_stack`");
                        if last_char == &'\\' {
                            chars_stack.push(ch);
                            continue;
                        } else {
                            attr_pos = AttrPos::Space;
                            let value = String::from_iter(chars_stack);
                            chars_stack = Vec::new();
                            value_stack.push(value)
                        }
                    } else {
                        chars_stack.push(ch)
                    }
                }
            },
            AttrPos::Space => {
                if ch != ' ' {
                    attr_pos = AttrPos::Key;
                    chars_stack.push(ch);
                }
            }
        }
    }

    let err_info = format!("cannot parse the attributes: {}", attr_str);
    let err_info = err_info.as_str();

    if !chars_stack.is_empty() {
        let str = String::from_iter(chars_stack);
        match attr_pos {
            AttrPos::Key => key_stack.push(str),
            AttrPos::Value(delimiter) => {
                if let None = delimiter {
                    value_stack.push(str);
                } else {
                    panic!("{}", err_info)
                }
            }
            _ => {}
        }
    }

    if key_stack.len() != value_stack.len() {
        panic!("{},{:?},{:?}", err_info, key_stack, value_stack)
    }

    let mut hashmap = HashMap::new();
    let len = key_stack.len();
    for _ in 0..len {
        hashmap.insert(
            key_stack.pop().expect(err_info),
            value_stack.pop().expect(err_info),
        );
    }
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
        crate::parse("<input disabled type=\"button\">");
    }
}
