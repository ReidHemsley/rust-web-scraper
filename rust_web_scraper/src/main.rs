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

    // List the options for the user to choose from
    println!("Select a CSS selector:");
    println!("1. #id");
    println!("2. .class");
    println!("3. element");
    
    // Prompt for the user's choice
    println!("Enter your choice:");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;
    let selector = match choice.trim() {
        "1" => Selector::parse("#id").expect("Failed to parse selector"),
        "2" => Selector::parse(".class").expect("Failed to parse selector"),
        "3" => Selector::parse("element").expect("Failed to parse selector"),
        _ => panic!("Invalid choice"),
    };

    // Iterate over elements matching our selector
    for element in document.select(&selector) {
        let text = element.text().collect::<Vec<_>>().join(" ");
        println!("{}", text);
    }

    Ok(())
}
