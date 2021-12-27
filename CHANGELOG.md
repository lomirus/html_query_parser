## v0.2.0

- Fix some fatal parse errors.
- Add `trim()` method to remove all empty text nodes.
- Add `html()` method to convert an element or nodes to html string.
- Add `Selector` struct, you can now create a selector by calling `Selector::from(str)`, which now support class selector(`.`), id selector(`#`), and tag selector(like `span`).
- Add `query(&Selector)` method to query a specific element.
- Add `query_all(&Selector)` method to query specific elements.

## v0.1.0 (2021-12-23)