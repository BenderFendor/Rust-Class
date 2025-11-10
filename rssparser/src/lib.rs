use quick_xml::escape::{escape, unescape};
use quick_xml::events::{Event, BytesText};
use quick_xml::*;
#[derive(Debug,Default)]
pub struct Article // Ok so my idea is to take an articles from feed's feed being vectors of articles
{
    pub author: String,
    pub date: String, // The standard libaray doesn't have a date type
    pub title: String,
    pub url: String,
    pub desc: String,
    pub categorys: Vec<String>,
}

#[derive(Debug,Default)]
pub struct Feed {
    pub name: String,
    pub articles: Vec<Article>,
}

impl Feed {
    pub fn parse(source: &str) -> Result<Feed, Box<dyn std::error::Error>>
    {
        let mut reader = quick_xml::Reader::from_str(source);
        
        reader.config_mut().trim_text(true);
        
        let mut buf = Vec::new();


        let mut feed = Feed::default(); // Use the final struct to hold the results
        let mut current_article = Article::default();
        
        let mut parsing_article = false;
        let mut last_tag_name: Option<&[u8]> = None; // To track which tag's text we are reading

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) => {
                    let name = e.name().as_ref();
                    last_tag_name = Some(name);

                    match name {
                        b"channel" => {}
                        b"item" => {
                            parsing_article = true;
                            current_article = Article::default(); // Reset for a new article
                        }
                        // Other start tags are handled by Event::Text
                        _ => {}
                    }
                }
                Ok(Event::Text(e)) => {
                    // Extract text content based on the last opened tag
                    let content = quick_xml::escape(unescape(e));

                    if let Some(tag_name) = last_tag_name {
                        match tag_name {
                            // Feed title (only if we are not inside an article)
                            b"title" if !parsing_article => {
                                feed.name = content;
                            }
                            // Article fields
                            b"title" if parsing_article => {
                                current_article.title = content;
                            }
                            b"link" if parsing_article => {
                                current_article.url = content;
                            }
                            b"pubDate" if parsing_article => {
                                current_article.date = content;
                            }
                            b"description" if parsing_article => {
                                current_article.desc = content;
                            }
                            b"dc:creator" if parsing_article => {
                                current_article.author = content;
                            }
                            b"category" if parsing_article => {
                                current_article.categorys.push(content);
                            }
                            _ => {}
                        }
                    }
                }
                Ok(Event::End(e)) => {
                    // When an item ends, push the collected article to the feed
                    if e.name().as_ref() == b"item" {
                        parsing_article = false;
                        feed.articles.push(current_article); // Store the completed article
                        current_article = Article::default(); // Reset, though not strictly needed here
                    }
                    last_tag_name = None; // Reset the last tag name after an end tag
                }
                Ok(Event::Eof) => {
                    // Correctly break and return the completed Feed
                    break Ok(feed);
                }
                Err(e) => return Err(Box::new(e)),
                _ => {}
            }
            buf.clear();
        }
    }
}
