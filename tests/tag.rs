#[test]
fn paired_tag() {
    html_query::parse(
        "
            <header></header>
            <div>Hello, world!</div>
            <footer></footer>",
    );
}
#[test]
fn void_tag() {
    html_query::parse("<div />");
    html_query::parse("<div/>");
}
#[test]
fn self_closing_tag() {
    html_query::parse("<img>");
}
#[test]
fn html_anatomy() {
    html_query::parse(
        "
            <!doctype html>
            <html>
                <head></head>
                <body></body>
            </html>",
    );
}
#[test]
fn comment_tag() {
    html_query::parse("<!-- comment -->");
    html_query::parse("<!--comment-->");
}
#[test]
fn attributes() {
    html_query::parse("<img src=\"example.png\" alt=example>");
    html_query::parse("<input disabled type=\"button\">");
}
