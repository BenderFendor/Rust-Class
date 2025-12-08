use quick_xml::Reader;
use quick_xml::events::Event;
use serde::{Deserialize, Serialize};
use tokio::time::{Duration, timeout};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    // This is specific to my implementation with my thesis and not part of RSS spec
    pub url: String,
    pub category: String,
    pub country: String,
    pub funding_type: String,
    pub bias_rating: String,
}

pub struct Sources {
    pub feeds: Vec<(Source, Feed)>,
}
impl Sources {
    pub fn new() -> Self {
        Sources { feeds: Vec::new() }
    }

    pub async fn fetch_from_urls(
        urls: Vec<(String, Source)>,
    ) -> Result<Sources, Box<dyn std::error::Error>> {
        let mut sources = Sources::new();
        let mut tasks = vec![];

        for (url, source) in urls {
            let task = tokio::spawn(async move {


                let copyurl = url.clone(); // master coder here.

                // Bound the entire fetch+parse operation to 5 seconds. As I had this hanging for a
                // good bit it was like 5 minutes
                match timeout(Duration::from_secs(5), async move {
                    match reqwest::get(&url).await {
                        Ok(response) => match response.text().await {
                            Ok(body) => match Feed::parse(&body) {
                                Ok(feed) => Some((source, feed)),
                                Err(e) => {
                                    eprintln!("Error parsing feed from {}: {}", url, e);
                                    None
                                }
                            },
                            Err(e) => {
                                eprintln!("Error fetching text from {}: {}", url, e);
                                None
                            }
                        },
                        Err(e) => {
                            eprintln!("Error fetching {}: {}", url, e);
                            None
                        }
                    }
                })
                .await
                {
                    Ok(result_opt) => result_opt,
                    Err(_) => {
                        eprintln!("Timeout fetching {} after 5s", copyurl);
                        None
                    }
                }
            });
            tasks.push(task);
        }

        for task in tasks {
            if let Ok(Some((source, feed))) = task.await {
                sources.feeds.push((source, feed));
            }
        }

        Ok(sources)
    }
}
#[derive(Debug, Default, Serialize, Deserialize)] // So the idea here was to get this parser to work with feeds with
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

#[derive(Debug, Default, Serialize, Deserialize)]
// I don't think you can derive display here so I would have to make my
// own method for displaying feeds and articles
#[allow(non_snake_case)] // I like having it the same is the RSS feed var names
pub struct Feed {
    pub name: String,
    pub articles: Vec<Article>,
    pub link: String,
    pub description: String,
    pub language: String,
    pub generator: String,
    pub copyright: String,
    pub managingEditor: String,
    pub webMaster: String,
    pub pubDate: String,
    pub lastBuildDate: String,
    pub categories: Vec<String>,
    pub docs: String,
    pub cloud: String,
    pub ttl: String,
    pub image: String,
    pub rating: String,
    pub skipHours: String,
    pub skipDays: String,
}
/* https://www.rssboard.org/rss-specification So that is the RSS Spec which is from 2009 so its
 * been "frozen" since then. The feed spec is for the rss 2.0 ver but there are other older ones as well*/

impl Feed {
    // Also I was looking it up and you can handle most of this with a Deserializer so I
    // wouldn't have to hard code it but it seems to simple and I think misses encoded tags
    fn handle_text_content(
        content: String,
        last_tag_name: &Option<String>,
        parsing_article: bool,
        feed: &mut Feed,
        current_article: &mut Article,
    ) {
        // this was needed for debug purposes but it adds to much clutter. Maybe should have print into log file
        // let content_preview: String = content.chars().take(40).collect();
        // println!(
        //     "tag: {} | content: {}",
        //     last_tag_name.as_deref().unwrap_or("none"),
        //     &content_preview
        // );

        if let Some(tag_name) = last_tag_name {
            match tag_name.as_str() {
                "title" if !parsing_article => {
                    // !parsing_article just means it's an feed object
                    feed.name = content;
                }
                "description" if !parsing_article => {
                    // It really seems stupid to make this many
                    // hardcoded if statements. Like it 99%
                    // boilerplate here
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
                "copyright" if !parsing_article => {
                    feed.copyright = content;
                }
                "managingEditor" if !parsing_article => {
                    feed.managingEditor = content;
                }
                "webMaster" if !parsing_article => feed.webMaster = content,
                "pubDate" if !parsing_article => {
                    feed.pubDate = content;
                }
                "lastBuildDate" if !parsing_article => {
                    feed.lastBuildDate = content;
                }
                "category" if !parsing_article => {
                    feed.categories.push(content);
                }
                "docs" if !parsing_article => {
                    feed.docs = content;
                }
                "cloud" if !parsing_article => {
                    feed.cloud = content;
                }
                "ttl" if !parsing_article => {
                    feed.ttl = content;
                }
                "image" if !parsing_article =>
                // This has sub-elements that I don't know how I
                // should deal with
                {
                    feed.image = content;
                }
                "rating" if !parsing_article => {
                    feed.rating = content;
                }
                "skipHours" if !parsing_article => {
                    feed.skipHours = content;
                }
                "skipDays" if !parsing_article => {
                    feed.skipDays = content;
                }
                "title" if parsing_article => {
                    current_article.title = content;
                }
                "link" if parsing_article => {
                    current_article.url = content;
                }
                "pubDate" if parsing_article => {
                    current_article.date = content;
                }
                "description" if parsing_article => {
                    current_article.desc = content;
                }
                tag if tag.ends_with(":creator") && parsing_article => {
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
        let mut parsing_image = false;
        let mut last_tag_name: Option<String> = None;

        fn push_enclosure_url(article: &mut Article, url: String) {
            if article.image_url.is_empty() {
                article.image_url = url;
            } else {
                article.image_url.push_str(", ");
                article.image_url.push_str(&url);
            }
        }

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
                        "image" if !parsing_article => {
                            parsing_image = true;
                        }
                        "enclosure" if parsing_article => {
                            if let Some(url) = e
                                .attributes()
                                .filter_map(|a| a.ok())
                                .find(|a| a.key.as_ref() == b"url")
                                .and_then(|a| a.unescape_value().ok())
                            {
                                push_enclosure_url(&mut current_article, url.to_string());
                            }
                        }
                        _ => {}
                    }
                }

                Ok(Event::Empty(e)) => {
                    let tag_name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    last_tag_name = Some(tag_name.clone());

                    if tag_name == "enclosure" && parsing_article {
                        if let Some(url) = e
                            .attributes()
                            .filter_map(|a| a.ok())
                            .find(|a| a.key.as_ref() == b"url")
                            .and_then(|a| a.unescape_value().ok())
                        {
                            push_enclosure_url(&mut current_article, url.to_string());
                        }
                    }
                }

                Ok(Event::Text(e)) => {
                    let content = e.decode()?.into_owned();

                    if parsing_image
                        && (last_tag_name.as_deref() == Some("url")
                            || last_tag_name.as_deref() == Some("image"))
                    {
                        feed.image = content;
                    } else if !parsing_image {
                        Self::handle_text_content(
                            content,
                            &last_tag_name,
                            parsing_article,
                            &mut feed,
                            &mut current_article,
                        );
                    }
                }

                Ok(Event::CData(e)) => {
                    let content = e.decode()?.into_owned();

                    if parsing_image
                        && (last_tag_name.as_deref() == Some("url")
                            || last_tag_name.as_deref() == Some("image"))
                    {
                        feed.image = content;
                    } else if !parsing_image {
                        Self::handle_text_content(
                            content,
                            &last_tag_name,
                            parsing_article,
                            &mut feed,
                            &mut current_article,
                        );
                    }
                }

                Ok(Event::End(e)) => {
                    let tag_name = String::from_utf8_lossy(e.name().as_ref()).to_string();

                    if tag_name == "item" {
                        parsing_article = false;
                        feed.articles.push(current_article);
                        current_article = Article::default();
                    } else if tag_name == "image" {
                        parsing_image = false;
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
        }
    }
}
