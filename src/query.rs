use crate::{Element, Node};

/// Used in `query()` and `query_all()` methods.
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
    /// The `selector` only supports type selector, ID selector and class selector.
    ///
    /// For example, `div#app`, `span` would be ok, but `.container > div`,
    /// `#app *` would get unexpected results.
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

    /// Check if the `element` matches the `selector`.
    pub fn matches(&self, element: &Element) -> bool {
        let mut matches = true;

        if self.tag != "" && element.name != self.tag {
            matches = false;
        }

        if self.class != "" {
            match element.attrs.get("class") {
                Some(class) => {
                    if &self.class != class {
                        matches = false;
                    }
                }
                None => {
                    if self.class != "" {
                        matches = false;
                    }
                }
            }
        }

        if self.id != "" {
            match element.attrs.get("id") {
                Some(id) => {
                    if &self.id != id {
                        matches = false;
                    }
                }
                None => {
                    if self.id != "" {
                        matches = false;
                    }
                }
            }
        }

        matches
    }
}

/// Implement `query()`, `query_all()` methods to `Vec<Node>` and `Element`.
pub trait Queryable {
    /// Query the node in `self` for the given selector.
    fn query(&self, selector: &Selector) -> Option<Element>;
    /// Query all the nodes in `self` for the given selector.
    fn query_all(&self, selector: &Selector) -> Vec<Element>;
}

impl Queryable for Vec<Node> {
    fn query(&self, selector: &Selector) -> Option<Element> {
        for node in self {
            if node.is_element() {
                let element = node.clone().try_into_element().unwrap();

                if selector.matches(&element) {
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
    fn query_all(&self, selector: &Selector) -> Vec<Element> {
        let mut elements = Vec::new();
        for node in self {
            if node.is_element() {
                let element = node.clone().try_into_element().unwrap();
                // Recursively traverse the descendants nodes
                let sub_elements = element.query_all(selector);
                elements.extend(sub_elements);
                // Check if this element matches. If so, push it to the `elements`
                if selector.matches(&element) {
                    elements.push(element);
                }
            }
        }
        elements
    }
}

impl Queryable for Element {
    fn query(&self, selector: &Selector) -> Option<Element> {
        self.children.query(selector)
    }
    fn query_all(&self, selector: &Selector) -> Vec<Element> {
        self.children.query_all(selector)
    }
}
