#[test]
fn paired_tag() {
    html_query_parser::parse("<p></p>");
    html_query_parser::parse("<div>Hello, world!</div>");
}

#[test]
fn void_tag() {
    html_query_parser::parse("<div />");
    html_query_parser::parse("<div/>");
}

#[test]
fn self_closing_tag() {
    html_query_parser::parse("<img>");
}

#[test]
fn comment_tag() {
    html_query_parser::parse("<!-- comment -->");
    html_query_parser::parse("<!--comment-->");
}

#[test]
fn attributes() {
    html_query_parser::parse("<img src=\"example.png\" alt=example>");
    html_query_parser::parse("<input disabled type=\"button\">");
}

#[test]
fn complex() {
    html_query_parser::parse(
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
            </div>
            <footer></footer>
        </body>
        </html>"#,
    );
}
