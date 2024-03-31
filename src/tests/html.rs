#[cfg(test)]
mod tests {
    use crate::html::HtmlParser;

    #[test]
    fn test_parse_example_com_html() {
        let parser = HtmlParser::new();
        let html_input = r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Example Domain</title>
        </head>
        <body>
            <div>
                <h1>Example Domain</h1>
                <p>This domain is for use in illustrative examples in documents. You may use this domain in literature without prior coordination or asking for permission.</p>
                <a href="http://www.iana.org/domains/example">More information...</a>
            </div>
        </body>
        </html>
        "#;
        let root_element = parser.parse_html(html_input);

        // Assuming the structure, the first significant child is `html`
        let html_element = &root_element.children.iter().find(|e| e.tag_name == Some("html".to_string())).expect("HTML element not found");

        // Within `html`, find the `body` element
        let body_element = html_element.children.iter().find(|e| e.tag_name == Some("body".to_string())).expect("Body element not found");

        // Now, find the first `div` within `body`
        let div_element = body_element.children.iter().find(|e| e.tag_name == Some("div".to_string())).expect("Div element not found");

        assert!(div_element.children.iter().any(|e| e.tag_name == Some("h1".to_string())));
        assert!(div_element.children.iter().any(|e| e.tag_name == Some("p".to_string())));
        assert!(div_element.children.iter().any(|e| e.tag_name == Some("a".to_string())));


}
}
