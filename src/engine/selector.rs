use super::html::HtmlElement;

pub struct Selector {
    tag_name: Option<String>,
    // Future fields for other types of selectors like class, id, etc.
}

impl Selector {
    pub fn from_tag_name(tag_name: &str) -> Self {
        Selector {
            tag_name: Some(tag_name.to_string()),
        }
    }

    // Annotate this method with a lifetime parameter `'a`
    // This ties the lifetime of the returned references to the lifetime of the input `element`
    pub fn select<'a>(&self, element: &'a HtmlElement) -> Selection<'a> {
        let mut selected: Vec<&'a HtmlElement> = Vec::new();
        self.select_recursive(element, &mut selected);
        Selection::new(selected)
    }

    // Ensure this helper method also uses the same lifetime parameter `'a`
    fn select_recursive<'a>(&self, element: &'a HtmlElement, selected: &mut Vec<&'a HtmlElement>) {
        if let Some(ref tag_name) = self.tag_name {
            if element.name == *tag_name {
                selected.push(element);
            }
        }
        // Iterate over children to search deeply
        for child in &element.children {
            self.select_recursive(child, selected);
        }
    }
}


#[derive(Debug)]
pub struct Selection<'a> {
    elements: Vec<&'a HtmlElement>,
}

impl<'a> Selection<'a> {
    // Constructor that takes selected elements
    pub fn new(elements: Vec<&'a HtmlElement>) -> Self {
        Selection { elements }
    }

    // Method to get the combined text of all selected elements
    pub fn text(&self) -> String {
        self.elements
            .iter()
            .map(|elem| elem.text.clone())
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn name (&self) -> String {
        self.elements
            .iter()
            .map(|elem| elem.name.clone())
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn class(&self) -> String {
        self.elements.iter()
            .filter_map(|elem| elem.attributes.get("class"))
            .cloned()
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn id(&self) -> Option<String> {
        self.elements.iter()
            .filter_map(|elem| elem.attributes.get("id"))
            .cloned()
            .next()
    }

    pub fn href(&self) -> Option<String> {
        self.elements.iter()
            .filter_map(|elem| elem.attributes.get("href"))
            .cloned()
            .next()
    }

    pub fn src(&self) -> Option<String> {
        self.elements.iter()
            .filter_map(|elem| elem.attributes.get("src"))
            .cloned()
            .next()
    }

}

