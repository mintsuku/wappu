#[cfg(test)]
mod parser_tests {
    use std::collections::HashMap;

    use crate::{Parser, Token};
    #[test]
    fn test_tokenize_simple_html() {
        let html = "<div>Hello, World!</div>";
        let tokens = Parser::new().tokenize(html);

        assert_eq!(tokens.len(), 3); // Expecting StartTag, Text, EndTag
        assert_eq!(tokens[0], Token::StartTag("div".to_string(), HashMap::new()));
        assert_eq!(tokens[1], Token::Text("Hello, World!".to_string()));
        assert_eq!(tokens[2], Token::EndTag("div".to_string()));
    }

    #[test]
    fn test_tokenize_with_attributes() {
        let html = r#"<a href="https://example.com">Link</a>"#;
        let tokens = Parser::new().tokenize(html);
        
        let mut attributes = HashMap::new();
        attributes.insert("href".to_string(), "https://example.com".to_string());

        assert_eq!(tokens.len(), 3); // StartTag, Text, EndTag
        assert_eq!(tokens[0], Token::StartTag("a".to_string(), attributes));
        assert_eq!(tokens[1], Token::Text("Link".to_string()));
        assert_eq!(tokens[2], Token::EndTag("a".to_string()));
    }

    #[test]
    fn test_tokenize_nested_html() {
        let html = "<div><p>Test</p></div>";
        let tokens = Parser::new().tokenize(html);

        assert_eq!(tokens.len(), 5); // StartTag(div), StartTag(p), Text, EndTag(p), EndTag(div)
        assert_eq!(tokens[0], Token::StartTag("div".to_string(), HashMap::new()));
        assert_eq!(tokens[1], Token::StartTag("p".to_string(), HashMap::new()));
        assert_eq!(tokens[2], Token::Text("Test".to_string()));
        assert_eq!(tokens[3], Token::EndTag("p".to_string()));
        assert_eq!(tokens[4], Token::EndTag("div".to_string()));
    }

    #[test]
    fn test_tokenize_with_multiple_attributes() {
        let html = r#"<input type="text" value="Sample" />"#;
        let tokens = Parser::new().tokenize(html);

        let mut attributes = HashMap::new();
        attributes.insert("type".to_string(), "text".to_string());
        attributes.insert("value".to_string(), "Sample".to_string());

        assert_eq!(tokens.len(), 1); // Single self-closing tag considered as StartTag
        assert_eq!(tokens[0], Token::StartTag("input".to_string(), attributes));
    }

}
