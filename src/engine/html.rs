use std::collections::HashMap;

use super::parser::{Parser, Token};

#[derive(Debug)]
pub struct HtmlElement {
    pub name: String,
    pub text: String,
    pub children: Vec<HtmlElement>,
    pub attributes: HashMap<String, String>,
}

impl HtmlElement {
    // Helper function to create a new HtmlElement
    pub fn new(name: String, attributes: HashMap<String, String>) -> Self {
        HtmlElement {
            name,
            text: String::new(),
            children: Vec::new(),
            attributes,
        }
    }
}

pub struct Html;

impl Html {
    pub fn new() -> Html {
        Html {}
    }
    
    pub fn parse_html(&self, html: &str) -> HtmlElement {
        let tokens = Parser::new().tokenize(html);
        let mut stack: Vec<HtmlElement> = Vec::new();

        // Initialize the root element with an empty HashMap for attributes
        let root = HtmlElement::new("root".to_string(), HashMap::new());
        stack.push(root);

        for token in tokens {
            match token {
                Token::StartTag(name, attributes) => {
                    let new_element = HtmlElement::new(name, attributes);
                    stack.push(new_element);
                },
                Token::Text(text) => {
                    if let Some(last) = stack.last_mut() {
                        // Append text to the current element's text.
                        if !last.text.is_empty() {
                            last.text.push_str(" ");
                        }
                        last.text.push_str(&text);
                    }
                },
                Token::EndTag(_) => {
                    if stack.len() > 1 {
                        let finished_element = stack.pop().unwrap();
                        if let Some(last) = stack.last_mut() {
                            last.children.push(finished_element);
                        }
                    }
                },
            }
        }

        // The first element in the stack is the root element containing the entire parsed HTML structure.
        stack.pop().unwrap()
    }
}
