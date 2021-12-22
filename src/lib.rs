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
    Doctype,
    /// Like `<!-- comment -->`
    Comment(String),
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
            Some(Self::from_comment(tag))
        } else if tag.starts_with("<!") {
            Some(Self::Doctype)
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

    #[inline]
    fn from_comment(comment: String) -> Self {
        Self::Comment(comment[4..comment.len() - 3].to_string())
    }
}

#[derive(Debug)]
enum Token {
    Tag(Tag),
    Text(String),
}

/// Let's take `<img src="example.png" alt=image>` for example.
enum AttrPos {
    /// This including `src`, `alt`
    Key,
    /// This including `=`
    Equal,
    /// This including `example.png`, `image`
    Value(Option<char>),
    /// This including ` `
    Space,
}

/// Valid `attr_str` like: `src="example.png" alt=example disabled`
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
                        if chars_stack.len() == 0 {
                            value_stack.push(String::new());
                            continue;
                        }
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
        panic!("{}", err_info)
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

fn html_to_stack(html: &str) -> Vec<Token> {
    let mut chars_stack = Vec::<char>::new();
    let mut token_stack = Vec::<Token>::new();
    let mut in_quotes: Option<char> = None;
    // More precisely: is in angle brackets
    let mut in_brackets = false;
    let mut in_comment = false;
    for ch in html.chars() {
        if let Some(quote) = in_quotes {
            if ch == quote {
                let last_char = chars_stack
                    .last()
                    .expect("cannot get the last char in chars stack")
                    .clone();
                if last_char != '\\' {
                    in_quotes = None;
                }
            }
            chars_stack.push(ch);
        } else if in_comment {
            chars_stack.push(ch);
            let len = chars_stack.len();
            if chars_stack[len - 3..len] == ['-', '-', '>'] {
                let comment = String::from_iter(chars_stack);
                chars_stack = Vec::new();
                let tag = Tag::from_comment(comment);
                token_stack.push(Token::Tag(tag));
                in_comment = false;
                in_brackets = false;
            }
        } else {
            match ch {
                '<' => {
                    in_brackets = true;
                    // In case of pushing empty text tokens to the stack
                    if chars_stack.len() != 0 {
                        // Turn the chars in `chars_stack` in to `String`
                        // and clean the chars stack.
                        let txt_text = String::from_iter(chars_stack);
                        chars_stack = Vec::new();
                        // Push the text we just got to the token stack.
                        token_stack.push(Token::Text(txt_text));
                    }
                    chars_stack.push(ch);
                }
                '>' => {
                    in_brackets = false;
                    chars_stack.push(ch);
                    // Turn the chars in `chars_stack` in to `String`
                    // and clean the chars stack.
                    let tag_text = String::from_iter(chars_stack);
                    chars_stack = Vec::new();
                    // Push the tag with the text we just got to the token stack.
                    let tag = Tag::from(tag_text.clone())
                        .expect(format!("Invalid tag: {}", tag_text).as_str());
                    token_stack.push(Token::Tag(tag));
                }
                '-' => {
                    chars_stack.push(ch);
                    if chars_stack.len() == 4 && chars_stack == ['<', '!', '-', '-'] {
                        in_comment = true;
                    }
                }
                _ => {
                    if in_brackets {
                        match ch {
                            '\'' => in_quotes = Some('\''),
                            '\"' => in_quotes = Some('\"'),
                            _ => {}
                        }
                    }
                    chars_stack.push(ch)
                }
            }
        }
    }
    token_stack
}

fn stack_to_dom(_: Vec<Token>) {}

pub fn parse(html: &str) {
    let stack = html_to_stack(html);
    println!("{:#?}", stack);
    let dom = stack_to_dom(stack);
    dom
}
