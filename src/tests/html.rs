#[cfg(test)]
mod tests {
    use crate::engine::html::Html;

    #[test]
    fn test_parse_complete_html() {
        let html = r#"
            <div class="container">
                <h1 id="title">Hello, World!</h1>
                <p>This is a test paragraph.</p>
                <ul>
                    <li>List item 1</li>
                    <li>List item 2</li>
                </ul>
            </div>
        "#;

        let parsed_html = Html::new().parse_html(html);

        // Check the root element
        assert_eq!(parsed_html.name, "root");
        assert!(parsed_html.attributes.is_empty());

        // Check the first child: <div class="container">
        let div = &parsed_html.children[0];
        assert_eq!(div.name, "div");
        assert_eq!(div.attributes.get("class").unwrap(), "container");

        // Check nested elements
        let h1 = &div.children[0];
        assert_eq!(h1.name, "h1");
        assert_eq!(h1.text, "Hello, World!");
        assert_eq!(h1.attributes.get("id").unwrap(), "title");

        let p = &div.children[1];
        assert_eq!(p.name, "p");
        assert_eq!(p.text, "This is a test paragraph.");

        let ul = &div.children[2];
        assert_eq!(ul.name, "ul");
        assert!(ul.attributes.is_empty());

        // Check list items within <ul>
        let li1 = &ul.children[0];
        assert_eq!(li1.name, "li");
        assert_eq!(li1.text, "List item 1");

        let li2 = &ul.children[1];
        assert_eq!(li2.name, "li");
        assert_eq!(li2.text, "List item 2");
    }
}
