use html_query_parser::{
    parse,
    trim::Trimable,
    html::Htmlifiable,
};

const HTML: &str = r#"
    <div>
        <span id="class">Hello</span>
        <span class="id">World</span>
    </div>"#;

#[test]
fn original_html() {
    let html = parse(HTML).into_html();
    assert_eq!(html, HTML);
}

#[test]
fn trimmed_html() {
    let html = parse(HTML).trim().into_html();
    assert_eq!(html, r#"<div><span id="class">Hello</span><span class="id">World</span></div>"#);
}
