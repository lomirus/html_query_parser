mod parse;

use std::collections::HashMap;

pub use parse::parse;

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
