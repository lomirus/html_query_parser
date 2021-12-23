use crate::{Element, Node};

#[derive(Debug)]
pub struct Selector {
    class: String,
    id: String,
    tag: String,
}

enum SelectorPos {
    Class,
    Id,
    Tag,
}

impl Selector {
    pub fn from(selector: &str) -> Self {
        let selector_chars = selector.trim().chars();
        let mut chars_stack = Vec::<char>::new();
        let mut selector_pos = SelectorPos::Tag;
        let mut selector = Selector {
            class: String::new(),
            id: String::new(),
            tag: String::new(),
        };

        for ch in selector_chars {
            match ch {
                '#' => {
                    let string = String::from_iter(chars_stack);
                    chars_stack = Vec::new();
                    match selector_pos {
                        SelectorPos::Class => selector.class = string,
                        SelectorPos::Id => selector.id = string,
                        SelectorPos::Tag => selector.tag = string,
                    }
                    selector_pos = SelectorPos::Id;
                }
                '.' => {
                    let string = String::from_iter(chars_stack);
                    chars_stack = Vec::new();
                    match selector_pos {
                        SelectorPos::Class => selector.class = string,
                        SelectorPos::Id => selector.id = string,
                        SelectorPos::Tag => selector.tag = string,
                    }
                    selector_pos = SelectorPos::Class;
                }
                _ => chars_stack.push(ch),
            }
        }
        let string = String::from_iter(chars_stack);
        match selector_pos {
            SelectorPos::Class => selector.class = string,
            SelectorPos::Id => selector.id = string,
            SelectorPos::Tag => selector.tag = string,
        }
        selector
    }
}

pub trait Queryable<T> {
    fn query(&self, selector: T) -> Option<Element>;
    fn query_all(&self, selector: T) -> Vec<Element>;
}

impl Queryable<&str> for Vec<Node> {
    /// Query the node for the given string selector.
    fn query(&self, selector: &str) -> Option<Element> {
        let selector = Selector::from(selector);
        self.query(&selector)
    }
    /// Query all the nodes for the given string selector.
    fn query_all(&self, selector: &str) -> Vec<Element> {
        let selector = Selector::from(selector);
        self.query_all(&selector)
    }
}

impl Queryable<&Selector> for Vec<Node> {
    /// Query the node for the given selector.
    fn query(&self, selector: &Selector) -> Option<Element> {
        for node in self {
            if node.is_element() {
                let element = node.clone().to_element().unwrap();
                let mut matched = true;
                if element.name != selector.tag {
                    matched = false;
                }
                match element.attrs.get("class") {
                    Some(class) => {
                        if &selector.class != class {
                            matched = false;
                        }
                    }
                    None => {
                        if selector.class != "" {
                            matched = false;
                        }
                    }
                }
                match element.attrs.get("id") {
                    Some(id) => {
                        if &selector.id != id {
                            matched = false;
                        }
                    }
                    None => {
                        if selector.id != "" {
                            matched = false;
                        }
                    }
                }
                if matched {
                    return Some(element);
                } else {
                    if let Some(elem) = element.query(selector) {
                        return Some(elem);
                    }
                }
            }
        }
        None
    }
    /// Query all the nodes for the given selector.
    fn query_all(&self, selector: &Selector) -> Vec<Element> {
        let mut elements = Vec::new();
        for node in self {
            if node.is_element() {
                let element = node.clone().to_element().unwrap();
                // Recursively traverse the descendants nodes
                let sub_elements = element.query_all(selector);
                elements.extend(sub_elements);
                // Check if this element matches. If so, push it to the `elements`
                let mut matched = true;
                if element.name != selector.tag {
                    matched = false;
                }
                match element.attrs.get("class") {
                    Some(class) => {
                        if &selector.class != class {
                            matched = false;
                        }
                    }
                    None => {
                        if selector.class != "" {
                            matched = false;
                        }
                    }
                }
                match element.attrs.get("id") {
                    Some(id) => {
                        if &selector.id != id {
                            matched = false;
                        }
                    }
                    None => {
                        if selector.id != "" {
                            matched = false;
                        }
                    }
                }
                if matched {
                    elements.push(element);
                }
            }
        }
        elements
    }
}

impl Queryable<&str> for Element {
    /// Query the node for the given string selector.
    fn query(&self, selector: &str) -> Option<Element> {
        let selector = Selector::from(selector);
        self.children.query(&selector)
    }
    /// Query all the nodes for the given string selector.
    fn query_all(&self, selector: &str) -> Vec<Element> {
        let selector = Selector::from(selector);
        self.children.query_all(&selector)
    }
}

impl Queryable<&Selector> for Element {
    /// Query the node for the given selector.
    fn query(&self, selector: &Selector) -> Option<Element> {
        self.children.query(selector)
    }
    /// Query all the nodes for the given selector.
    fn query_all(&self, selector: &Selector) -> Vec<Element> {
        self.children.query_all(selector)
    }
}
