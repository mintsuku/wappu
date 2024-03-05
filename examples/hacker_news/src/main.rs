use wappu::{select_by_class, select_by_tag_name, HtmlParser, Selector, WappuClient};

#[tokio::main]
async fn main() {
    let client = WappuClient::new();
    let response = client
        .get("https://news.ycombinator.com", None)
        .await
        .expect("Failed to fetch content");
    let root_element = HtmlParser::new().parse_html(&response.text());

    let article_elements = select_by_class!(&root_element, "athing").elements;
    let detail_elements = select_by_class!(&root_element, "subtext").elements;

    for (index, article) in article_elements.into_iter().enumerate() {

        let title_line_selection = select_by_class!(article, "titleline");
        let title_link = title_line_selection
            .elements
            .first()
            .expect("Title line not found");

        let a_tag_selection = select_by_tag_name!(title_link, "a");
        let a_tag = a_tag_selection.elements.first().expect("Link not found");

        let title = &a_tag.text;
        let url = a_tag.attributes.get("href").expect("URL not found");

        let details = detail_elements.get(index).expect("Details not found");
        let points = select_by_class!(details, "score")
            .elements
            .first()
            .map_or("0 points", |e| &e.text);
        let author = select_by_class!(details, "hnuser")
            .elements
            .first()
            .map_or("Unknown author", |e| &e.text);

        let comments = select_by_tag_name!(details, "a")
            .elements
            .iter()
            .rev()
            .find_map(|e| {
                if e.attributes.contains_key("href")
                    && e.attributes.get("href").unwrap().contains("item?id=")
                {
                    Some(e.text.as_str())
                } else {
                    None
                }
            })
            .unwrap_or("0 comments");

        println!("Title: {}", title);
        println!("URL: {}", url);
        println!("Points: {}", points);
        println!("Author: {}", author);
        println!("Comments: {}", comments);
        println!("---");
    }
}
