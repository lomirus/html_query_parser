use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Token {
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
    /// Any text
    Text(String),
}

impl Token {
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

    fn to_node(&self) -> Node {
        match &self {
            Self::Start(tag_name, attrs) => Node::Element {
                name: tag_name.clone(),
                attrs: attrs.clone(),
                children: Vec::new(),
            },
            Self::End(tag_name) => Node::Element {
                name: tag_name.clone(),
                attrs: HashMap::new(),
                children: Vec::new(),
            },
            Self::Closing(tag_name, attrs) => Node::Element {
                name: tag_name.clone(),
                attrs: attrs.clone(),
                children: Vec::new(),
            },
            Self::Doctype => Node::Doctype,
            Self::Comment(comment) => Node::Comment(comment.clone()),
            Self::Text(text) => Node::Text(text.clone()),
        }
    }
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

#[derive(Debug)]
pub enum Node {
    Element {
        name: String,
        attrs: HashMap<String, String>,
        children: Vec<Node>,
    },
    Text(String),
    Comment(String),
    Doctype,
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
                            attr_pos = AttrPos::Space;
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
        panic!(
            "{}\nkey:\t{:?}\nvalue:\t{:?}",
            err_info, key_stack, value_stack
        )
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
                let tag = Token::from_comment(comment);
                token_stack.push(tag);
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
                    let tag = Token::from(tag_text.clone())
                        .expect(format!("Invalid tag: {}", tag_text).as_str());
                    token_stack.push(tag);
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

fn stack_to_dom(token_stack: Vec<Token>) -> Vec<Node> {
    let mut nodes = Vec::new();

    let mut i = 0;
    while i < token_stack.len() {
        match &token_stack[i] {
            Token::Start(start_name, attrs) => {
                // Find the matched closing tag from end to start.
                let end_pos = token_stack.iter().rev().position(|x| {
                    if let Token::End(end_name) = x {
                        start_name == end_name
                    } else {
                        false
                    }
                });
                if let Some(j_rev) = end_pos {
                    // You found a paired tags like `<p></p>`
                    let j = token_stack.len() - 1 - j_rev;
                    let children = stack_to_dom(token_stack[i + 1..j].to_vec());
                    nodes.push(Node::Element {
                        name: start_name.clone(),
                        attrs: attrs.clone(),
                        children,
                    });
                    i = j;
                } else {
                    // Else you found an unpaired tags like `<img>`
                    nodes.push(token_stack[i].to_node());
                }
            }
            Token::End(name) => panic!("unexpected end tag: {}", name),
            _ => nodes.push(token_stack[i].to_node()),
        }
        i += 1;
    }

    nodes
}

pub fn parse(html: &str) -> Vec<Node> {
    let stack = html_to_stack(html);
    let dom = stack_to_dom(stack);
    dom
}
