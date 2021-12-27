[![Crates.io](https://img.shields.io/crates/v/html_query_parser)](https://crates.io/crates/html_query_parser)

# HTML Query Parser

Pure, simple and elegant HTML parser and query selector.

## Examples

### Parse HTML segment/document

```rust
use html_query_parser::parse;

let document = parse("<!doctype html><html><head></head><body></body></html>");
println!("{:#?}", document);
```

Output:

```rust
[
    Doctype,
    Element {
        name: "html",
        attrs: {},
        children: [
            Element {
                name: "head",
                attrs: {},
                children: [],
            },
            Element {
                name: "body",
                attrs: {},
                children: [],
            },
        ],
    },
]
```

### Query an element by classname

```rust
use html_query_parser::{parse, Queryable, Selector};

let html = r#"
    <div>
        <span>Hello</span>
        <span>World</span>
        <div class="last">Last Element</div>
    </div>"#;
let nodes = parse(html);
let selector = Selector::from(".last");
let element = nodes.query(&selector).unwrap();
```

### Query all elements by tag

```rust
use html_query_parser::{parse, Queryable, Selector};

let html = r#"
    <div>
        <span>Hello</span>
        <span>World</span>
        <div class="last">Last Element</div>
    </div>"#;
let nodes = parse(html);
let selector = Selector::from("span");
let elements = nodes.query_all(&selector);
```

### Edit the HTML

```rust
use html_query_parser::{parse, Trimable, Htmlifiable};

let html = r#"
    <div>
        <span>Hello</span>
        <span>World</span>
        <div class="last">Last Element</div>
    </div>"#;
let html = parse(html).trim().html();
println!("{}", html);
```

Output:

```log
<div><span>Hello</span><span>World</span><div class="last">Last Element</div></div>
```