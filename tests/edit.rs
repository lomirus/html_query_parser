use std::collections::HashMap;

use html_query_parser::{parse, Editable, Htmlifiable, Node, Selector};

const HTML: &str = r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <title>Document</title>
    </head>
    <body>
    </body>
    </html>"#;

const TEST_HTML: &str = r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <title>Document</title>
    </head>
    <body>
    <script>console.log("Hello World")</script></body>
    </html>"#;

#[test]
fn trimmed_html() {
    let body_selector = Selector::from("body");
    let script = Node::Element {
        name: "script".to_string(),
        attrs: HashMap::new(),
        children: vec![Node::Text(r#"console.log("Hello World")"#.to_string())],
    };
    let html = parse(HTML).insert_to(&body_selector, script).html();
    assert_eq!(html, TEST_HTML);
}
