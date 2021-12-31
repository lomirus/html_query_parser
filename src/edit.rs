use crate::{Element, Node, Selector};

/// Trim or insert elements into the DOM.
pub trait Editable {
    /// Remove all empty text nodes from `self`.
    fn trim(self) -> Self;
    /// Insert `node` as the last child to all elements that matches the `selector`.
    fn insert_to(&mut self, selector: &Selector, target: Node) -> &mut Self;
}

impl Editable for Vec<Node> {
    fn trim(self) -> Self {
        let mut nodes: Vec<Node> = Vec::new();
        for node in self {
            match node {
                Node::Element {
                    name,
                    attrs,
                    children,
                } => nodes.push(Node::Element {
                    name,
                    attrs,
                    children: children.trim(),
                }),
                Node::Text(text) => {
                    if text.trim() != "" {
                        nodes.push(Node::Text(text));
                    }
                }
                Node::Comment(_) => {}
                Node::Doctype => nodes.push(node),
            }
        }
        nodes
    }
    fn insert_to(&mut self, selector: &Selector, target: Node) -> &mut Self {
        for node in self.iter_mut() {
            if let Node::Element {
                name,
                attrs,
                children,
            } = node
            {
                children.insert_to(selector, target.clone());
                if selector.matches(&Element {
                    name: name.clone(),
                    attrs: attrs.clone(),
                    children: vec![],
                }) {
                    children.push(target.clone());
                }
            }
        }
        self
    }
}

impl Editable for Element {
    fn trim(self) -> Self {
        Element {
            name: self.name,
            attrs: self.attrs,
            children: self.children.trim(),
        }
    }
    fn insert_to(&mut self, selector: &Selector, target: Node) -> &mut Self {
        self.children.insert_to(selector, target.clone());
        if selector.matches(self) {
            self.children.push(target);
        }
        self
    }
}
