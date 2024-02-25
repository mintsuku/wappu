use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Token {
    StartTag(String, HashMap<String, String>),
    EndTag(String),
    Text(String),
}

pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Parser {}
    }

    pub fn tokenize(&self, html: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut i = 0;
        while i < html.len() {
            if html.as_bytes()[i] == b'<' {
                if html.as_bytes()[i + 1] == b'/' {
                    // It's an end tag
                    let start = i + 2; // Start after "</"
                    i += 2; // Move past "</"
                    while i < html.len() && html.as_bytes()[i] != b'>' { i += 1; }
                    let tag_name = &html[start..i];
                    tokens.push(Token::EndTag(tag_name.to_string()));
                    i += 1; // Skip over ">"
                } else {
                    // It's a start tag
                    let start = i + 1; // Start after "<"
                    i += 1; // Move past "<"
                    while i < html.len() && html.as_bytes()[i] != b'>' { i += 1; }
                    let tag_contents = &html[start..i];
                    let (tag_name, attributes_string) = tag_contents.split_once(' ').unwrap_or((tag_contents, ""));
                    let attributes = Parser::parse_attributes(attributes_string);
                    tokens.push(Token::StartTag(tag_name.to_string(), attributes));
                    i += 1; // Skip over ">"
                }
            } else {
                // It's text
                let start = i;
                while i < html.len() && html.as_bytes()[i] != b'<' { i += 1; }
                let text = &html[start..i].trim();
                if !text.is_empty() {
                    tokens.push(Token::Text(text.to_string()));
                }
            }
        }
        tokens
    }

    fn parse_attributes(attributes_string: &str) -> HashMap<String, String> {
        attributes_string.split_whitespace()
            .filter_map(|part| {
                let mut parts = part.splitn(2, '=');
                if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                    Some((key.to_string(), value.trim_matches(|c: char| c == '\"' || c == '\'').to_string()))
                } else {
                    None
                }
            })
            .collect()
    }
}
