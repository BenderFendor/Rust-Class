use rssparser::Feed;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::get("https://truthout.org/feed")
        .await?
        .text()
        .await?;

    match Feed::parse(&body) {
        Ok(feed) => {
            println!("Successfully parsed the feed.");
            println!("{:#?}", feed);
        }
        Err(e) => {
            eprintln!("Error parsing the feed: {}", e);
        }
    }

    Ok(())
}
