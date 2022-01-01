## Warning

This repository has been deprecated, please use the [html-editor](https://github.com/lomirus/html-editor) instead.

If you want to have the ownership of this crate name, please contact me at <lomirus@qq.com> so I can transfer it to you.

[![Crates.io](https://img.shields.io/crates/v/html_query_parser)](https://crates.io/crates/html_query_parser)

# HTML Query Parser

Pure, simple and elegant HTML parser and query selector.

## Examples

### Parse HTML segment/document

```rust
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
// let html = r#"..."#
let nodes = parse(html);
let selector: Selector = Selector::from(".last");
let element: Element = nodes.query(&selector).unwrap();
```

### Query all elements by tag

```rust
// let html = r#"..."#
let nodes = parse(html);
let selector: Selector = Selector::from("span");
let elements: Vec<Element> = nodes.query_all(&selector);
```

### Edit the HTML

```rust
// let html = r#"..."#
let a: String = parse(html).trim().html();
let b: String = parse(html).insert_to(&selector, node).html();
let c: String = parse(html).remove_by(&selector).html();
```

You can find more examples in the [documentation](https://docs.rs/html_query_parser/latest/html_query_parser/).

## Changelog

See in [CHANGELOG.md](CHANGELOG.md)
