#[cfg(test)]
mod tests {
    use crate::{engine::selector::Selector, select_by_class, select_by_tag_name, HtmlParser, WappuClient}; // Assuming the HtmlElement struct is defined in a module named `html`

    #[test]
    fn test_html_parsing_and_selection() {
        let parser = HtmlParser::new();
        let html = r#"
            <div class="content">
                <h1 id="title">Test Title</h1>
                <a href="https://example.com">Example Link</a>
                <img src="image.jpg" alt="An image">
            </div>
        "#;

        let parsed_html = parser.parse_html(html);

        // Test selecting by tag name
        let mut h1_selector = Selector::new();
        let h1_selection = h1_selector.from_tag_name("h1").select(&parsed_html);
        assert_eq!(h1_selection.text(), "Test Title");
        assert_eq!(h1_selection.tag_name(), Some("h1".to_string()));
        assert_eq!(h1_selection.id(), Some("title".to_string()));

        let mut a_selector = Selector::new();
        let a_selection = a_selector.from_tag_name("a").select(&parsed_html);
        assert_eq!(a_selection.text(), "Example Link");
        assert_eq!(a_selection.href(), Some("https://example.com".to_string()));

        let mut img_selector = Selector::new();
        let img_selection = img_selector.from_tag_name("img").select(&parsed_html);
        assert_eq!(img_selection.src(), Some("image.jpg".to_string()));
    }

    #[test]
    fn test_html_parsing_and_selection_with_nested_elements() {
        let parser = HtmlParser::new();
        let html = r#"
            <div class="content">
                <h1 id="title">Test Title</h1>
                <a href="https://example.com">Example Link</a>
                <div>
                    <p>Some text</p>
                    <a href="https://example.com">Another link</a>
                </div>
            </div>
        "#;

        let parsed_html = parser.parse_html(html);

        // Test selecting by tag name
        let mut p_selector = Selector::new();
        let p_selection = p_selector.from_tag_name("p").select(&parsed_html);
        assert_eq!(p_selection.text(), "Some text");

        let mut a_selector = Selector::new();
        let a_selection = a_selector.from_tag_name("a").select(&parsed_html);
        assert_eq!(a_selection.text(), "Example Link Another link");
    }

    #[tokio::test]
    async fn test_selection_request() {
        let client = WappuClient::new();
        let result = client.get("https://doc.rust-lang.org/book/").await.unwrap();
        let html = HtmlParser::new().parse_html(&result);

        let mut header_selector = Selector::new();
        let header = header_selector.from_class_name("header").select(&html);

        assert_eq!(header.text().trim(), "The Rust Programming Language");
    }

    #[tokio::test]
    async fn test_selection_macro() {
        let client = WappuClient::new();
        let result = client.get("https://doc.rust-lang.org/book/").await.unwrap();
        let html = HtmlParser::new().parse_html(&result);

        let header = select_by_class!(&html, "header");
        
        // No need to manually create or mutate a `Selector` instance here
        assert_eq!(header.text().trim(), "The Rust Programming Language");
    }

    #[tokio::test]
    async fn test_selection_by_tag_macro() {
        let client = WappuClient::new();
        let result = client.get("https://doc.rust-lang.org/book/").await.unwrap();
        let html = HtmlParser::new().parse_html(&result);

        let header = select_by_tag_name!(&html, "h1");
        
        // No need to manually create or mutate a `Selector` instance here
        assert_eq!(header.text().trim(), "The Rust Programming Language");
    }
}
