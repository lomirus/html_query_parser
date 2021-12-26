mod parse;
pub mod trim;
pub mod query;

use std::collections::HashMap;

pub use parse::parse;

#[derive(Debug, Clone)]
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

#[derive(Debug)]
pub struct Element {
    pub name: String,
    pub attrs: HashMap<String, String>,
    pub children: Vec<Node>,
}
