use crate::{Element, Node};

pub trait Htmlifiable {
    fn html(&self) -> String;
}

impl Htmlifiable for Element {
    fn html(&self) -> String {
        if self.attrs.len() == 0 {
            return format!(
                "<{}>{}</{}>",
                self.name,
                self.children.html(),
                self.name
            );
        }
        let attrs = self
            .attrs
            .iter()
            .map(|(k, v)| {
                format!(
                    "{}=\"{}\"",
                    k,
                    v.replace("\"", "\\\"").replace("\'", "\\\'")
                )
            })
            .collect::<Vec<_>>()
            .join(" ");
        format!(
            "<{} {}>{}</{}>",
            self.name,
            attrs,
            self.children.html(),
            self.name
        )
    }
}

impl Htmlifiable for Node {
    fn html(&self) -> String {
        match self {
            Node::Element { .. } => self.try_element().unwrap().html(),
            Node::Text(text) => text.to_string(),
            Node::Comment(comment) => format!("<!--{}-->", comment),
            Node::Doctype => "<!DOCTYPE html>".to_string(),
        }
    }
}

impl Htmlifiable for Vec<Node> {
    fn html(&self) -> String {
        let mut html = String::new();
        for node in self {
            html.push_str(node.html().as_str());
        }
        html
    }
}
