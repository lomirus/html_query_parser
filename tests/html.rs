use html_query_parser::{
    parse,
    trim::Trimable,
    html::Htmlifiable,
};

const HTML: &str = r#"
    <div>
        <span>Hello</span>
        <div class="last">Last Element</div>
    </div>"#;

#[test]
fn html() {
    let html = parse(HTML).trim().to_html();
    println!("{}", html);
}
