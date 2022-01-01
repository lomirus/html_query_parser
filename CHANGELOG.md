## v0.3.0 (2022-01-01)

- Feat: Add `insert_to`, `remove_by` methods;
- Feat: Add `matches` method for selector;
- Fix: the end tag of void tags will be attached when converted to html code;
- Docs: More detailed documentation and examples.

## v0.2.1 (2021-12-27)

- Fix some documentation problems
- Simplify the `try_element()` method

## v0.2.0 (2021-12-27)

- Fix some fatal parse errors.
- Add `trim()` method to remove all empty text nodes.
- Add `html()` method to convert an element or nodes to html string.
- Add `Selector` struct, you can now create a selector by calling `Selector::from(str)`, which now supports class selector(`.`), id selector(`#`), and tag selector(like `span`).
- Add `query(&Selector)` method to query a specific element.
- Add `query_all(&Selector)` method to query specific elements.

## v0.1.0 (2021-12-23)