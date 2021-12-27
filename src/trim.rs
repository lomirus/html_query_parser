use crate::{Element, Node};

/// Implement `trim()` method to `Vec<Node>` and `Element`.
pub trait Trimable {
    /// Remove all empty text nodes from `self`.
    fn trim(self) -> Self;
}

impl Trimable for Vec<Node> {
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
}

impl Trimable for Element {
    fn trim(self) -> Self {
        Element {
            name: self.name,
            attrs: self.attrs,
            children: self.children.trim(),
        }
    }
}
