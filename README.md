# Wappu: A Rust Web Scraping Library

Wappu is a comprehensive web scraping library written in Rust, designed for ease of use and performance. It integrates seamlessly HTTP client capabilities with powerful HTML parsing functionalities, allowing users to fetch and parse web content efficiently.

## Features

- **Asynchronous HTTP Requests**: Fetch web pages asynchronously with a simple-to-use HTTP client.
- **HTML Parsing**: Easily parse and query HTML documents to extract relevant data.
- **Flexible Selectors**: Use CSS-like selectors to pinpoint and extract elements from parsed HTML.
- **Error Handling**: Robust error handling for both network requests and HTML parsing.

## Getting Started

### Prerequisites

Ensure you have Rust and Cargo installed on your system. Wappu requires Rust version 1.40 or newer.

### Installation

Add Wappu to your project's `Cargo.toml`:

```toml
[dependencies]
<<<<<<< HEAD
wappu = "0.2.3"
=======
wappu = "0.2.2"
reqwest = "0.11.24"
>>>>>>> 3f3fac55368f5a7c07fed99c671ab1df1effb743
```

### Quick Example

Here's a quick example to fetch and parse the title of example.com:

```rust
use wappu::{WappuClient, HtmlParser, Selector};
use tokio;

#[tokio::main]
async fn main() {
    let client = WappuClient::new();
    let html_content = client.get("http://example.com", None).await.expect("Failed to fetch content");

<<<<<<< HEAD
    let parsed_html = Html::new().parse_html(&html_content.text());
    let title_selector = Selector::from_tag_name("h1");
=======
    let parsed_html = HtmlParser::new().parse_html(&html_content);
    let mut selector = Selector::new();
    let title_selector = selector.from_tag_name("title");
>>>>>>> 3f3fac55368f5a7c07fed99c671ab1df1effb743
    let title_selection = title_selector.select(&parsed_html);
    let title_text = title_selection.text();

    println!("Title: {}", title_text);
}
```

## Documentation

For detailed documentation, including API reference and advanced usage, visit [Wappu Documentation](#). (Not yet done)

## Contributing

Contributions are welcome! Please see our [Contributing Guide](CONTRIBUTING.md) for more details.

## License

Wappu is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Thanks to the Rust community for the invaluable resources and support.
- Special thanks to [httpbin](https://httpbin.org/) for providing HTTP request & response service, making it easier to test HTTP client functionalities.
