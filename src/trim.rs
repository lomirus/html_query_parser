use crate::{Node, Element};

trait Trimable {
    fn trim(self) -> Self;
}

impl Trimable for Vec<Node> {
    fn trim(self) -> Self {
        let mut nodes = Vec::new();
        for node in self {
            if node.is_element() {
                nodes.push(node);
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