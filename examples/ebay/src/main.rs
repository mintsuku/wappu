use wappu::{select_by_class, select_by_tag_name, HtmlParser, Selector, WappuClient};

#[tokio::main]
async fn main() {
    let client = WappuClient::new();
    let response = client
        .get(
            "https://www.ebay.com/sch/i.html?_nkw=rust+programming+book",
            None,
        )
        .await
        .expect("Failed to fetch content");
    let root_element = HtmlParser::new().parse_html(&response.text());

    let listings = select_by_class!(&root_element, "s-item");


    for listing in listings.into_iter() {
    

        let title_element = select_by_class!(listing, "s-item__title")
            .elements
            .first()
            .cloned();

        if let Some(title_element_owned) = title_element {
            let title_span = select_by_tag_name!(&title_element_owned, "span")
                .elements
                .first()
                .map(|span| span.text.clone())
                .unwrap_or_else(|| "No title found".to_string());

            let price = select_by_class!(&listing, "s-item__price")
                .elements
                .first()
                .map(|e| e.text.clone())
                .unwrap_or_else(|| "No price found".to_string());

            let link = select_by_tag_name!(&listing, "a")
                .elements
                .first()
                .and_then(|e| e.attributes.get("href").cloned())
                .unwrap_or_else(|| "No URL found".to_string());

            println!("Title: {}", title_span);
            println!("Price: {}", price);
            println!("Link: {}", link);
            println!("---");
        }
    }
}
