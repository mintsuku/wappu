use super::html::HtmlElement;

#[macro_export]
macro_rules! select_by_tag_name {
    ($element:expr, $tag_name:expr) => {
        Selector::new().from_tag_name($tag_name).select($element)
    };
}

#[macro_export]
macro_rules! select_by_class {
    ($element:expr, $class_name:expr) => {
        Selector::new().from_class_name($class_name).select($element)
    };
}


pub struct Selector {
    tag_name: Option<String>,
    class_name: Option<String>,
}

impl Selector {
    pub fn new() -> Self {
        Selector {
            tag_name: None,
            class_name: None,
        }
    }


    pub fn from_tag_name(&mut self, tag_name: &str) -> &mut Self {
        self.tag_name = Some(tag_name.to_string());
        self
    }

    pub fn from_class_name(&mut self, class_name: &str) -> &mut Self {
        self.class_name = Some
        (class_name.to_string());
        self
    }

    pub fn select<'a>(&self, element: &'a HtmlElement) -> Selection<'a> {
        let mut selected: Vec<&'a HtmlElement> = Vec::new();
        self.select_recursive(element, &mut selected);
        Selection::new(selected)
    }

    pub fn select_first<'a>(&self, element: &'a HtmlElement) -> Option<&'a HtmlElement> {
        let mut selected: Vec<&'a HtmlElement> = Vec::new();
        self.select_recursive(element, &mut selected);
        selected.into_iter().next()
    }

    fn select_recursive<'a>(&self, element: &'a HtmlElement, selected: &mut Vec<&'a HtmlElement>) {
        if self.tag_name.as_ref() == element.tag_name.as_ref() || self.matches_class(element) {
            selected.push(element);
        }
        for child in &element.children {
            self.select_recursive(child, selected);
        }
    }

    fn matches_class(&self, element: &HtmlElement) -> bool {
        match self.class_name {
            Some(ref class_name) => {
                element.attributes.get("class").map_or(false, |classes| {
                    classes.split_whitespace().any(|class| class == class_name)
                })
            }
            None => false,
        }
    }
}

#[derive(Debug)]
pub struct Selection<'a> {
    pub elements: Vec<&'a HtmlElement>,
}

impl<'a> IntoIterator for Selection<'a> {
    type Item = &'a HtmlElement;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}

impl<'a> Selection<'a> {
    pub fn new(elements: Vec<&'a HtmlElement>) -> Self {
        Selection { elements }
    }

    pub fn text(&self) -> String {
        self.elements
            .iter()
            .map(|elem| elem.text.clone())
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn tag_name(&self) -> Option<String> {
        self.elements
            .iter()
            .filter_map(|elem| elem.tag_name.as_ref())
            .next()
            .cloned()
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn class(&self) -> Option<String> {
        self.elements
            .iter()
            .filter_map(|elem| elem.attributes.get("class"))
            .next()
            .cloned()
    }

    pub fn id(&self) -> Option<String> {
        self.elements
            .iter()
            .filter_map(|elem| elem.attributes.get("id"))
            .next()
            .cloned()
    }

    pub fn href(&self) -> Option<String> {
        self.elements
            .iter()
            .filter_map(|elem| elem.attributes.get("href"))
            .next()
            .cloned()
    }

    pub fn src(&self) -> Option<String> {
        self.elements
            .iter()
            .filter_map(|elem| elem.attributes.get("src"))
            .next()
            .cloned()
    }
}
