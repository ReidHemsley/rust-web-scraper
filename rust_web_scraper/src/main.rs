use reqwest;
use scraper::{Html, Selector};
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Prompt the user for the website URL
    print!("Enter the website URL: ");
    io::stdout().flush()?; // Make sure the prompt is visible immediately
    let mut url = String::new();
    io::stdin().read_line(&mut url)?;
    let url = url.trim();

    // Send a GET request to the URL
    let html = reqwest::get(url).await?.text().await?;

    // Parse the HTML document
    let document = Html::parse_document(&html);

    // Prompt the user for the CSS selector
    print!("Enter the CSS selector: ");
    io::stdout().flush()?; // Make sure the prompt is visible immediately
    let mut selector_str = String::new();
    io::stdin().read_line(&mut selector_str)?;
    let selector_str = selector_str.trim();

    // Parse the CSS selector
    let selector = Selector::parse(selector_str).expect("Failed to parse selector");

    // Iterate over elements matching our selector
    for element in document.select(&selector) {
        let text = element.text().collect::<Vec<_>>().join(" ");
        println!("{}", text);
    }

    Ok(())
}