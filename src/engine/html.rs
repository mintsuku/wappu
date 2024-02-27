extern crate html5ever;
extern crate markup5ever_rcdom as rcdom;

use html5ever::{parse_document, tendril::TendrilSink};
use markup5ever_rcdom::{Handle, NodeData, RcDom};
use std::collections::HashMap;

pub struct HtmlParser;

impl HtmlParser {
    pub fn new() -> Self {
        HtmlParser {}
    }

    pub fn parse_html(&self, input: &str) -> HtmlElement {
        let dom = parse_document(RcDom::default(), Default::default()).one(input);

        HtmlElement::from_dom(&dom.document)
    }
}

#[derive(Debug)]
pub struct HtmlElement {
    pub tag_name: Option<String>,
    pub text: String,
    pub children: Vec<HtmlElement>,
    pub attributes: HashMap<String, String>,
}

impl HtmlElement {
    fn from_dom(handle: &Handle) -> Self {
        match handle.data {
            NodeData::Document => {
                let children = handle
                    .children
                    .borrow()
                    .iter()
                    .map(HtmlElement::from_dom)
                    .collect();

                HtmlElement {
                    tag_name: None,
                    text: String::new(),
                    children,
                    attributes: HashMap::new(),
                }
            }
            NodeData::Element { ref name, ref attrs, .. } => {
                let tag_name = Some(name.local.to_string());
                let attributes = attrs.borrow().iter().map(|attr| {
                    (attr.name.local.to_string(), attr.value.to_string())
                }).collect();

                let mut children = Vec::new();
                let mut text = String::new();
                for child in handle.children.borrow().iter() {
                    let child_element = HtmlElement::from_dom(child);
                    // Append child text to the parent element's text if the child is a text node.
                    if child_element.tag_name.is_none() {
                        text += &child_element.text;
                    } else {
                        children.push(child_element);
                    }
                }

                HtmlElement {
                    tag_name,
                    text, // Now contains the aggregated text of its child text nodes.
                    children,
                    attributes,
                }
            }
            NodeData::Text { ref contents } => HtmlElement {
                tag_name: None,
                text: contents.borrow().to_string(),
                children: vec![],
                attributes: HashMap::new(),
            },
            _ => HtmlElement {
                tag_name: None,
                text: String::new(),
                children: vec![],
                attributes: HashMap::new(),
            },
        }
    }
}
