#[test]
fn paired_tag() {
    html_query::parse("<p></p>");
    html_query::parse("<div>Hello, world!</div>");
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
fn comment_tag() {
    html_query::parse("<!-- comment -->");
    html_query::parse("<!--comment-->");
}

#[test]
fn attributes() {
    html_query::parse("<img src=\"example.png\" alt=example>");
    html_query::parse("<input disabled type=\"button\">");
}

#[test]
fn complex() {
    html_query::parse(
        r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta http-equiv="X-UA-Compatible" content="IE=edge">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Document</title>
        </head>
        <body>
            <header>Example</header>
            <div>
                <input value="<p value='haha'></p>" disable placeholder=input>
                <input value="\"\"''/>">
            </div>
            <footer></footer>
        </body>
        </html>"#,
    );
}
