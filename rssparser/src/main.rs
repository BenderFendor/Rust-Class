use rssparser::{Source, Sources};
use std::collections::BTreeMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load JSON sources file
    let json_data = std::fs::read_to_string("src/rss_sources.json")?;
    let sources_map: BTreeMap<String, serde_json::Value> = serde_json::from_str(&json_data)?;

    let mut urls_with_metadata = vec![];

    // Extract URL and metadata for each source
    for (_name, data) in sources_map.iter() {
        if let Some(url) = data.get("url").and_then(|v| v.as_str()) {
            let source = Source {
                url: url.to_string(),
                category: data
                    .get("category")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                country: data
                    .get("country")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                funding_type: data
                    .get("funding_type")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                bias_rating: data
                    .get("bias_rating")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
            };
            urls_with_metadata.push((url.to_string(), source));
        }
    }

    println!("Fetching {} RSS feeds...", urls_with_metadata.len());

    // Fetch and parse all feeds concurrently
    match Sources::fetch_from_urls(urls_with_metadata).await {
        Ok(sources) => {
            println!("\nSuccessfully parsed {} feeds", sources.feeds.len());

            for (source, feed) in &sources.feeds {
                println!("\n--- {} ---", feed.name);
                println!("Category: {}", source.category);
                println!("Country: {}", source.country);
                println!("Bias Rating: {}", source.bias_rating);
                println!("Articles: {}", feed.articles.len());

                for article in feed.articles.iter().take(3) {
                    println!("  - {}", article.title);
                }
            }
        }
        Err(e) => {
            eprintln!("Error fetching sources: {}", e);
        }
    }

    Ok(())
}
