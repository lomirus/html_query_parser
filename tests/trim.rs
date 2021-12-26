use html_query_parser::{
    parse,
    trim::Trimable,
};

const HTML: &str = r#"
    <div>
        <span>Hello</span>
        <div class="last">Last Element</div>
    </div>"#;

#[test]
fn trim() {
    let nodes = parse(HTML).trim();
    println!("{:#?}", nodes);
}
