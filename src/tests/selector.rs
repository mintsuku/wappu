#[cfg(test)]
mod selector_tests {
    use crate::Selector;
    use crate::engine::html::HtmlElement;
    use std::collections::HashMap;

    // Helper function to create an HtmlElement with attributes
    fn create_element(name: &str, attributes: HashMap<&str, &str>) -> HtmlElement {
        HtmlElement::new(
            name.to_string(),
            attributes.into_iter().map(|(k, v)| (k.to_string(), v.to_string())).collect(),
        )
    }

    #[test]
    fn test_select_by_tag_name() {
        let root = create_element("div", HashMap::new());
        let child1 = create_element("span", HashMap::new());
        let child2 = create_element("span", HashMap::from([("class", "test")]));

        let mut root_with_children = root;
        root_with_children.children.push(child1);
        root_with_children.children.push(child2);

        let selector = Selector::from_tag_name("span");
        let selection = selector.select(&root_with_children);

        assert_eq!(selection.len(), 2);
    }

    #[test]
    fn test_selection_text() {
        let mut root = create_element("div", HashMap::new());
        root.children.push(HtmlElement {
            name: "p".to_string(),
            text: "Test text".to_string(),
            children: Vec::new(),
            attributes: HashMap::new(),
        });

        let selector = Selector::from_tag_name("p");
        let selection = selector.select(&root);

        assert_eq!(selection.text(), "Test text");
    }

    #[test]
    fn test_selection_class() {
        let mut root = create_element("div", HashMap::new());
        root.children.push(create_element("p", HashMap::from([("class", "test")])));
        
        let selector = Selector::from_tag_name("p");
        let selection = selector.select(&root);

        assert_eq!(selection.class(), "test");
    }

    #[test]
    fn test_selection_id() {
        let mut root = create_element("div", HashMap::new());
        root.children.push(create_element("p", HashMap::from([("id", "unique")])));
        
        let selector = Selector::from_tag_name("p");
        let selection = selector.select(&root);

        assert_eq!(selection.id(), Some("unique".to_string()));
    }

    #[test]
    fn test_selection_href() {
        let mut root = create_element("a", HashMap::new());
        root.children.push(create_element("a", HashMap::from([("href", "http://example.com")])));
        
        let selector = Selector::from_tag_name("a");
        let selection = selector.select(&root);

        assert_eq!(selection.href(), Some("http://example.com".to_string()));
    }

    #[test]
    fn test_selection_src() {
        let mut root = create_element("img", HashMap::new());
        root.children.push(create_element("img", HashMap::from([("src", "image.png")])));
        
        let selector = Selector::from_tag_name("img");
        let selection = selector.select(&root);

        assert_eq!(selection.src(), Some("image.png".to_string()));
    }
}


#[cfg(test)]
mod client_integration_tests {
    use crate::{WappuClient, Selector, Html};

    #[tokio::test]
    async fn test_fetch_and_parse_title_from_example_com() {
        let client = WappuClient::new();
        // Fetch the HTML content from example.com
        let html_content = client.get("http://example.com").await.expect("Failed to fetch content");

        // Parse the fetched HTML to create an HtmlElement structure
        let parsed_html = Html::new().parse_html(&html_content);

        // Create a Selector for the <title> tag
        let title_selector = Selector::from_tag_name("h1");

        // Select the <title> element from the parsed HTML
        let title_selection = title_selector.select(&parsed_html);

        // Get the text content of the <title> element
        let title_text = title_selection.text();

        // Print the title text for debugging
        println!("Title: {}", title_text);

        assert_eq!(title_text, "Example Domain");
    }
}
