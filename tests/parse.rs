use html_query_parser::parse;

#[test]
fn paired_tag() {
    parse("<p></p>");
    parse("<div>Hello, world!</div>");
}

#[test]
fn void_tag() {
    parse("<div />");
    parse("<div/>");
}

#[test]
fn self_closing_tag() {
    parse("<img>");
}

#[test]
fn comment_tag() {
    parse("<!-- comment -->");
    parse("<!--comment-->");
}

#[test]
fn attributes() {
    parse("<img src=\"example.png\" alt=example>");
    parse("<input disabled type=\"button\">");
}

#[test]
fn matched() {
    parse(
        r#"
        <span>
            <span>
                <span></span>
            </span>
        </span>"#,
    );
    parse(
        r#"
        <span></span>
        <span></span>
        <span></span>"#,
    );
    parse(
        r#"
        <span>
            <span></span>
        </span>
        <span></span>"#,
    );
}

#[test]
fn complex() {
    parse(
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
                <!-- Nothing is true -->
                <!-- Everything is permitted -->
                <!-- <p></p> -->
                <!------------->
                <a b="" c="d"></a>
            </div>
            <footer></footer>
        </body>
        </html>"#,
    );
}
