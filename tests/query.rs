use html_query_parser::{parse, query::Queryable};

const HTML: &str = r#"
    <div>
        <span>Hello</span>
        <span>World</span>
        <div class="last">Last Element</div>
    </div>"#;

#[test]
fn nodes_query() {
    let nodes = parse(HTML);
    let element = nodes.query("span").unwrap();
    println!("{:?}", element);
}


#[test]
fn nodes_query_all() {
    let nodes = parse(HTML);
    let elements = nodes.query_all("span");
    println!("{:?}", elements);
}

#[test]
fn element_query() {
    let nodes = parse(HTML);
    let node = nodes.into_iter().nth(1).unwrap();
    let element = node.to_element().unwrap().query("span").unwrap();
    println!("{:?}", element);
}

#[test]
fn element_query_all() {
    let nodes = parse(HTML);
    let node = nodes.into_iter().nth(1).unwrap();
    let elements = node.to_element().unwrap().query_all("span");
    println!("{:?}", elements);
}

#[test]
fn class_query() {
    let nodes = parse(HTML);
    let element = nodes.query(".last").unwrap();
    println!("{:?}", element);
}