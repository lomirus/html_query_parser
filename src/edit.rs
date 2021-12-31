use crate::{Element, Node, Selector};

/// Implement `trim()` method to `Vec<Node>` and `Element`.
pub trait Editable {
    /// Remove all empty text nodes from `self`.
    fn trim(self) -> Self;
    /// Insert `node` as a child in the front of all elements that matched the `selector`.
    fn insert_before_of(&mut self, selector: Selector, node: Node);
    /// Insert `node` as a child in the back of all elements that matched the `selector`.
    fn insert_after_of(&mut self, selector: Selector, node: Node);
    /// Insert `node` before all elements that matched the `selector`.
    fn insert_before(&mut self, selector: Selector, node: Node);
    /// Insert `node` after all elements that matched the `selector`.
    fn insert_after(&mut self, selector: Selector, node: Node);
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
    fn insert_before_of(&mut self, selector: Selector, node: Node) {}
    fn insert_after_of(&mut self, selector: Selector, node: Node) {}
    fn insert_before(&mut self, selector: Selector, node: Node) {}
    fn insert_after(&mut self, selector: Selector, node: Node) {}
}

impl Editable for Element {
    fn trim(self) -> Self {
        Element {
            name: self.name,
            attrs: self.attrs,
            children: self.children.trim(),
        }
    }
    fn insert_before_of(&mut self, selector: Selector, node: Node) {}
    fn insert_after_of(&mut self, selector: Selector, node: Node) {}
    fn insert_before(&mut self, selector: Selector, node: Node) {}
    fn insert_after(&mut self, selector: Selector, node: Node) {}
}
