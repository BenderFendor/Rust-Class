use quick_xml::Reader;
use quick_xml::events::Event;

#[derive(Debug, Default)] // So the idea here was to get this parser to work with feeds with
// articles as vecs of articles
pub struct Article {
    pub author: String,
    pub date: String,
    pub title: String,
    pub url: String,
    pub desc: String,
    pub categories: Vec<String>,
    pub content: String,
    pub image_url: String,
}

#[derive(Debug, Default)] // I don't think you can derive display here so I would have to make my
// own method for displaying feeds and articles
pub struct Feed {
    pub name: String,
    pub articles: Vec<Article>,
    pub link: String,
    pub description: String,
    pub language: String,
    pub generator: String,
}

impl Feed {
    fn handle_text_content(
        content: String,
        last_tag_name: &Option<String>,
        parsing_article: bool,
        feed: &mut Feed,
        current_article: &mut Article,
    ) 
    {
        let content_preview: String = content.chars().take(40).collect();
        println!(
            "tag: {} | content: {}",
            last_tag_name.as_deref().unwrap_or("none"),
            &content_preview
        );

        if let Some( tag_name) = last_tag_name {
            match tag_name.as_str() {
                "title" if !parsing_article => {
                    feed.name = content;
                }
                "description" if !parsing_article => {
                    feed.description = content;
                }
                "link" if !parsing_article => {
                    feed.link = content;
                }
                "language" if !parsing_article => {
                    feed.language = content;
                }
                "generator" if !parsing_article => {
                    feed.generator = content;
                }
                "title" if parsing_article => {
                    current_article.title = content;
                }
                "link" if parsing_article => {
                    current_article.url = content;
                }
                "pubdate" if parsing_article => {
                    current_article.date = content;
                }
                "description" if parsing_article => {
                    current_article.desc = content;
                }
                // Can you not make this a wildcard? so like *:creater
                "dc:creator" if parsing_article => {
                    current_article.author = content;
                }
                "category" if parsing_article => {
                    current_article.categories.push(content);
                }
                "content:encoded" if parsing_article => {
                    current_article.content = content;
                }
                "truthout:authors" if parsing_article => {
                    if current_article.author.is_empty() {
                        current_article.author = content;
                    }
                }
                _ => {}
            }
        } else {
            println!(
                "warning: no tag for content: {}",
                &content[..content.len().min(40)]
            );
        }
    }
    pub fn parse(source: &str) -> Result<Feed, Box<dyn std::error::Error>> {
        let mut reader = Reader::from_str(source);
        reader.config_mut().trim_text(true);

        let mut buf = Vec::new();
        let mut feed = Feed::default();
        let mut current_article = Article::default();
        let mut parsing_article = false;
        let mut last_tag_name: Option<String> = None;

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) => {
                    let tag_name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    last_tag_name = Some(tag_name.clone());

                    match tag_name.as_str() {
                        "item" => {
                            parsing_article = true;
                            current_article = Article::default();
                        }
                        _ => {}
                    }
                }

                Ok(Event::Text(e)) => {
                    let content = e.decode()?.into_owned();
                    Self::handle_text_content(
                        content,
                        &last_tag_name,
                        parsing_article,
                        &mut feed,
                        &mut current_article,
                    );
                }
                Ok(Event::CData(e)) => {
                    let content = e.decode()?.into_owned();
                    Self::handle_text_content(
                        content,
                        &last_tag_name,
                        parsing_article,
                        &mut feed,
                        &mut current_article,
                    );
                }
                Ok(Event::Empty(e)) => {
                    let name_bytes = e.name();
                    let tag_name = String::from_utf8_lossy(name_bytes.as_ref());

                    if parsing_article && tag_name == "enclosure" {
                        for attr in e.attributes() {
                            if let Ok(attr) = attr {
                                if attr.key.as_ref() == b"url" {
                                    let url = String::from_utf8_lossy(&attr.value).to_string();
                                    if !current_article.image_url.is_empty() {
                                        current_article.image_url.push_str(", ");
                                    }
                                    current_article.image_url.push_str(&url);
                                }
                            }
                        }
                    }
                }
                Ok(Event::End(e)) => {
                    if e.name().as_ref() == b"item" {
                        parsing_article = false;
                        feed.articles.push(current_article);
                        current_article = Article::default(); // This clears categories for next article
                    }
                }

                Ok(Event::Eof) => {
                    break Ok(feed);
                }

                Err(e) => {
                    return Err(Box::new(e) as Box<dyn std::error::Error>);
                }

                _ => {}
            }
            buf.clear();
        }
    }
}
