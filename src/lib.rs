mod attrs;
mod token;

use std::collections::HashMap;
use token::Token;

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

impl Node {
    pub fn is_element(&self) -> bool {
        match self {
            Node::Element { .. } => true,
            _ => false,
        }
    }

    pub fn to_element(self) -> Result<Element, &'static str>  {
        match self {
            Node::Element {
                name,
                attrs,
                children,
            } => Ok(Element {
                name,
                attrs,
                children,
            }),
            _ => Err("not an element"),
        }
    }
}

pub struct Selector;

impl Selector {
    pub fn from(selector: &str) -> Self {
        Selector
    }
}

pub struct Element {
    pub name: String,
    pub attrs: HashMap<String, String>,
    pub children: Vec<Node>,
}

pub trait Queryable<T> {
    fn query(&self, selector: T) -> Option<Element>;
    fn query_all(&self, selector: T) -> Vec<Element>;
}

impl Queryable<&str> for Vec<Node> {
    /// Query the node for the given string selector.
    fn query(&self, selector: &str) -> Option<Element> {
        let selector = Selector::from(selector);
        self.query(selector)
    }
    /// Query all the nodes for the given string selector.
    fn query_all(&self, selector: &str) -> Vec<Element> {
        let selector = Selector::from(selector);
        self.query_all(selector)
    }
}

impl Queryable<Selector> for Vec<Node> {
    /// Query the node for the given selector.
    fn query(&self, selector: Selector) -> Option<Element> {
        todo!()
    }
    /// Query all the nodes for the given selector.
    fn query_all(&self, selector: Selector) -> Vec<Element> {
        todo!()
    }
}

impl Queryable<&str> for Element {
    /// Query the node for the given string selector.
    fn query(&self, selector: &str) -> Option<Element> {
        todo!()
    }
    /// Query all the nodes for the given string selector.
    fn query_all(&self, selector: &str) -> Vec<Element> {
        todo!()
    }
}

impl Queryable<Selector> for Element {
    /// Query the node for the given selector.
    fn query(&self, selector: Selector) -> Option<Element> {
        todo!()
    }
    /// Query all the nodes for the given selector.
    fn query_all(&self, selector: Selector) -> Vec<Element> {
        todo!()
    }
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

/// Parse the html string and return a `Vec` of `Node`.
///
/// Example:
///
/// ```
/// use html_query_parser::parse;
///
/// // Parse a segment.
/// let segment = parse(r#"<p class="content">Hello, world!</p>"#);
/// println!("{:#?}", segment);
///
/// // Or you can parse a whole html file.
/// let document = parse("<!doctype html><html><head></head><body></body></html>");
/// println!("{:#?}", document);
/// ```
/// Output:
/// ```log
/// [
///     Element {
///         name: "p",
///         attrs: {
///             "class": "content",
///         },
///         children: [
///             Text(
///                 "Hello, world!",
///             ),
///         ],
///     },
/// ]
/// [
///     Doctype,
///     Element {
///         name: "html",
///         attrs: {},
///         children: [
///             Element {
///                 name: "head",
///                 attrs: {},
///                 children: [],
///             },
///             Element {
///                 name: "body",
///                 attrs: {},
///                 children: [],
///             },
///         ],
///     },
/// ]
/// ```
pub fn parse(html: &str) -> Vec<Node> {
    let stack = html_to_stack(html);
    let dom = stack_to_dom(stack);
    dom
}
