use quick_xml::events::Event;
use quick_xml::Reader;

#[derive(Debug, Default)] // So the idea here was to get this parser to work with feeds with
                          // articles as vecs of articles
pub struct Article {
    pub author: String,
    pub date: String,
    pub title: String,
    pub url: String,
    pub desc: String,
    pub categories: Vec<String>, 
}

#[derive(Debug, Default)] // I don't think you can derive display here so I would have to make my
                          // own method for displaying feeds and articles
pub struct Feed {
    pub name: String,
    pub articles: Vec<Article>,
}

impl Feed {
    pub fn parse(source: &str) -> Result<Feed, Box<dyn std::error::Error>> {
        let mut reader = Reader::from_str(source);
        reader.config_mut().trim_text(true);
        
        let mut buf = Vec::new();
        let mut feed = Feed::default();
        let mut current_article = Article::default();
        let mut parsing_article = false;
        let mut last_tag_name: Option<Vec<u8>> = None; 

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) => {
                    let name = e.name().as_ref().to_vec(); // Convert to owned vector
                    last_tag_name = Some(name.clone());

                    match name.as_slice() {
                        b"item" => {
                            parsing_article = true;
                            current_article = Article::default();
                        }
                        _ => {}
                    }
                }
                
                Ok(Event::Text(e)) => {
                    // Something about byte text which is a pain to work with 
                    let content = e.decode()?.into_owned();
                    
                    if let Some(ref tag_name) = last_tag_name {
                        match tag_name.as_slice() {
                            b"title" if !parsing_article => {
                                feed.name = content;
                            }
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
                                current_article.categories.push(content);
                            }
                            _ => {}
                        }
                    }
                }
                
                Ok(Event::End(e)) => {
                    if e.name().as_ref() == b"item" {
                        parsing_article = false;
                        feed.articles.push(current_article);
                        current_article = Article::default();
                    }
                    last_tag_name = None;                 }
                
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
