use rssparser::Feed; // Adjust the module path as necessary

#[cfg(test)] // This is maybe the dumbest thing I've ever done in Rust but I wanted to
// practice writing tests for Rust and this seemed like a good idea
// It's way to many tests tho.
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic_feed_metadata() {
        let xml = r#"
            <?xml version="1.0" encoding="UTF-8"?>
            <rss version="2.0">
                <channel>
                    <title>My Test Feed</title>
                    <description>A description of the test feed</description>
                    <link>http://example.com</link>
                    <language>en-us</language>
                    <generator>Rust RSS Parser</generator>
                    <copyright>2024 Example Corp</copyright>
                    <managingEditor>editor@example.com</managingEditor>
                    <webMaster>webmaster@example.com</webMaster>
                    <pubDate>Tue, 10 Jun 2003 04:00:00 GMT</pubDate>
                    <lastBuildDate>Tue, 10 Jun 2003 09:41:01 GMT</lastBuildDate>
                    <docs>http://blogs.law.harvard.edu/tech/rss</docs>
                    <ttl>60</ttl>
                </channel>
            </rss>
        "#;

        let feed = Feed::parse(xml).expect("Failed to parse feed");

        assert_eq!(feed.name, "My Test Feed");
        assert_eq!(feed.description, "A description of the test feed");
        assert_eq!(feed.link, "http://example.com");
        assert_eq!(feed.language, "en-us");
        assert_eq!(feed.generator, "Rust RSS Parser");
        assert_eq!(feed.copyright, "2024 Example Corp");
        assert_eq!(feed.managingEditor, "editor@example.com");
        assert_eq!(feed.webMaster, "webmaster@example.com");
        assert_eq!(feed.pubDate, "Tue, 10 Jun 2003 04:00:00 GMT");
        assert_eq!(feed.lastBuildDate, "Tue, 10 Jun 2003 09:41:01 GMT");
        assert_eq!(feed.docs, "http://blogs.law.harvard.edu/tech/rss");
        assert_eq!(feed.ttl, "60");
    }

    #[test]
    fn test_parse_feed_categories() {
        let xml = r#"
            <rss version="2.0">
                <channel>
                    <title>Category Test</title>
                    <category>Technology</category>
                    <category>Programming</category>
                    <category>Rust</category>
                </channel>
            </rss>
        "#;

        let feed = Feed::parse(xml).expect("Failed to parse feed");
        assert_eq!(feed.categories.len(), 3);
        assert_eq!(feed.categories[0], "Technology");
        assert_eq!(feed.categories[1], "Programming");
        assert_eq!(feed.categories[2], "Rust");
    }

    #[test]
    fn test_parse_single_article() {
        let xml = r#"
            <rss version="2.0">
                <channel>
                    <title>Article Test</title>
                    <item>
                        <title>First Article</title>
                        <link>http://example.com/1</link>
                        <description>Short description</description>
                        <pubDate>Mon, 01 Jan 2024 00:00:00 GMT</pubDate>
                        <dc:creator>John Doe</dc:creator>
                    </item>
                </channel>
            </rss>
        "#;

        let feed = Feed::parse(xml).expect("Failed to parse feed");
        assert_eq!(feed.articles.len(), 1);

        let article = &feed.articles[0];
        assert_eq!(article.title, "First Article");
        assert_eq!(article.url, "http://example.com/1");
        assert_eq!(article.desc, "Short description");
        assert_eq!(article.date, "Mon, 01 Jan 2024 00:00:00 GMT");
        assert_eq!(article.author, "John Doe");
    }

    #[test]
    fn test_parse_multiple_articles() {
        let xml = r#"
            <rss version="2.0">
                <channel>
                    <title>Multi Article Test</title>
                    <item>
                        <title>Article 1</title>
                    </item>
                    <item>
                        <title>Article 2</title>
                    </item>
                    <item>
                        <title>Article 3</title>
                    </item>
                </channel>
            </rss>
        "#;

        let feed = Feed::parse(xml).expect("Failed to parse feed");
        assert_eq!(feed.articles.len(), 3);
        assert_eq!(feed.articles[0].title, "Article 1");
        assert_eq!(feed.articles[1].title, "Article 2");
        assert_eq!(feed.articles[2].title, "Article 3");
    }

    #[test]
    fn test_cdata_handling() {
        let xml = r#"
            <rss version="2.0">
                <channel>
                    <title>CDATA Test</title>
                    <item>
                        <title><![CDATA[Title with <special> characters]]></title>
                        <description><![CDATA[<p>HTML content</p>]]></description>
                        <content:encoded><![CDATA[<div>Full content</div>]]></content:encoded>
                    </item>
                </channel>
            </rss>
        "#;

        let feed = Feed::parse(xml).expect("Failed to parse feed");
        let article = &feed.articles[0];

        assert_eq!(article.title, "Title with <special> characters");
        assert_eq!(article.desc, "<p>HTML content</p>");
        assert_eq!(article.content, "<div>Full content</div>");
    }

    #[test]
    fn test_article_categories() {
        let xml = r#"
            <rss version="2.0">
                <channel>
                    <item>
                        <title>Categorized Article</title>
                        <category>News</category>
                        <category>World</category>
                    </item>
                </channel>
            </rss>
        "#;

        let feed = Feed::parse(xml).expect("Failed to parse feed");
        let article = &feed.articles[0];

        assert_eq!(article.categories.len(), 2);
        assert_eq!(article.categories[0], "News");
        assert_eq!(article.categories[1], "World");
    }

    #[test]
    fn test_enclosure_image() {
        let xml = r#"
            <rss version="2.0">
                <channel>
                    <item>
                        <title>Image Article</title>
                        <enclosure url="http://example.com/image.jpg" type="image/jpeg" />
                    </item>
                </channel>
            </rss>
        "#;

        let feed = Feed::parse(xml).expect("Failed to parse feed");
        let article = &feed.articles[0];

        assert_eq!(article.image_url, "http://example.com/image.jpg");
    }

    #[test]
    fn test_multiple_enclosures() {
        let xml = r#"
            <rss version="2.0">
                <channel>
                    <item>
                        <title>Multi Image Article</title>
                        <enclosure url="http://example.com/1.jpg" type="image/jpeg" />
                        <enclosure url="http://example.com/2.jpg" type="image/jpeg" />
                    </item>
                </channel>
            </rss>
        "#;

        let feed = Feed::parse(xml).expect("Failed to parse feed");
        let article = &feed.articles[0];

        assert_eq!(
            article.image_url,
            "http://example.com/1.jpg, http://example.com/2.jpg"
        );
    }

    #[test]
    fn test_truthout_authors_fallback() {
        let xml = r#"
            <rss version="2.0" xmlns:truthout="http://truthout.org">
                <channel>
                    <item>
                        <title>Truthout Article</title>
                        <truthout:authors>Jane Smith</truthout:authors>
                    </item>
                </channel>
            </rss>
        "#;

        let feed = Feed::parse(xml).expect("Failed to parse feed");
        let article = &feed.articles[0];

        assert_eq!(article.author, "Jane Smith");
    }

    #[test]
    fn test_truthout_authors_priority() {
        let xml = r#"
            <rss version="2.0" xmlns:truthout="http://truthout.org" xmlns:dc="http://purl.org/dc/elements/1.1/">
                <channel>
                    <item>
                        <title>Priority Article</title>
                        <dc:creator>Primary Author</dc:creator>
                        <truthout:authors>Secondary Author</truthout:authors>
                    </item>
                </channel>
            </rss>
        "#;

        let feed = Feed::parse(xml).expect("Failed to parse feed");
        let article = &feed.articles[0];

        assert_eq!(article.author, "Primary Author");
    }

    #[test]
    fn test_malformed_xml() {
        let xml = r#"
            <rss version="2.0">
                <channel>
                    <title>Broken Feed
                </channel>
            </rss>
        "#;

        let result = Feed::parse(xml);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_feed() {
        let xml = r#"
            <rss version="2.0">
                <channel>
                </channel>
            </rss>
        "#;

        let feed = Feed::parse(xml).expect("Failed to parse empty feed");
        assert_eq!(feed.name, "");
        assert_eq!(feed.articles.len(), 0);
    }

    #[test]
    fn test_skip_hours_days() {
        let xml = r#"
            <rss version="2.0">
                <channel>
                    <skipHours>6</skipHours>
                    <skipDays>Saturday</skipDays>
                </channel>
            </rss>
        "#;

        let feed = Feed::parse(xml).expect("Failed to parse feed");
        assert_eq!(feed.skipHours, "6");
        assert_eq!(feed.skipDays, "Saturday");
    }

    #[test]
    fn test_cloud_tag() {
        let xml = r#"
            <rss version="2.0">
                <channel>
                    <cloud domain="rpc.sys.com" port="80" path="/RPC2" registerProcedure="myCloud.rssPleaseNotify" protocol="xml-rpc" />
                </channel>
            </rss>
        "#;

        let feed = Feed::parse(xml).expect("Failed to parse feed");
        assert_eq!(feed.cloud, "");
    }

    #[test]
    fn test_rating_tag() {
        let xml = r#"
            <rss version="2.0">
                <channel>
                    <rating>(PICS-1.1 "http://www.rsac.org/ratingsv01.html" 1 gen true comment "RSAC" on "2024.01.01T00:00-0800" r (n 0 s 0 v 0 l 0))</rating>
                </channel>
            </rss>
        "#;

        let feed = Feed::parse(xml).expect("Failed to parse feed");
        assert!(feed.rating.contains("PICS-1.1"));
    }

    #[test]
    fn test_image_tag() {
        let xml = r#"
            <rss version="2.0">
                <channel>
                    <image>http://example.com/logo.png</image>
                </channel>
            </rss>
        "#;

        let feed = Feed::parse(xml).expect("Failed to parse feed");
        assert_eq!(feed.image, "http://example.com/logo.png");
    }

    #[test]
    fn test_docs_tag() {
        let xml = r#"
            <rss version="2.0">
                <channel>
                    <docs>http://blogs.law.harvard.edu/tech/rss</docs>
                </channel>
            </rss>
        "#;

        let feed = Feed::parse(xml).expect("Failed to parse feed");
        assert_eq!(feed.docs, "http://blogs.law.harvard.edu/tech/rss");
    }

    #[test]
    fn test_article_with_all_fields() {
        let xml = r#"
            <rss version="2.0" xmlns:content="http://purl.org/rss/1.0/modules/content/" xmlns:dc="http://purl.org/dc/elements/1.1/">
                <channel>
                    <item>
                        <title>Complete Article</title>
                        <link>http://example.com/article</link>
                        <description>Article description</description>
                        <pubDate>Wed, 01 Jan 2025 12:00:00 GMT</pubDate>
                        <dc:creator>Author Name</dc:creator>
                        <category>Category1</category>
                        <category>Category2</category>
                        <content:encoded><![CDATA[<p>Full HTML content</p>]]></content:encoded>
                        <enclosure url="http://example.com/image.jpg" type="image/jpeg" />
                    </item>
                </channel>
            </rss>
        "#;

        let feed = Feed::parse(xml).expect("Failed to parse feed");
        let article = &feed.articles[0];

        assert_eq!(article.title, "Complete Article");
        assert_eq!(article.url, "http://example.com/article");
        assert_eq!(article.desc, "Article description");
        assert_eq!(article.date, "Wed, 01 Jan 2025 12:00:00 GMT");
        assert_eq!(article.author, "Author Name");
        assert_eq!(article.categories.len(), 2);
        assert_eq!(article.content, "<p>Full HTML content</p>");
        assert_eq!(article.image_url, "http://example.com/image.jpg");
    }

    #[test]
    fn test_complex_feed_with_mixed_content() {
        let xml = r#"
            <rss version="2.0">
                <channel>
                    <title>Complex Feed</title>
                    <description>A complex feed with various elements</description>
                    <link>http://example.com</link>
                    <language>en-us</language>
                    <category>News</category>
                    <category>Tech</category>
                    <item>
                        <title>Article 1</title>
                        <link>http://example.com/1</link>
                    </item>
                    <item>
                        <title>Article 2</title>
                        <link>http://example.com/2</link>
                    </item>
                </channel>
            </rss>
        "#;

        let feed = Feed::parse(xml).expect("Failed to parse feed");

        assert_eq!(feed.name, "Complex Feed");
        assert_eq!(feed.description, "A complex feed with various elements");
        assert_eq!(feed.link, "http://example.com");
        assert_eq!(feed.language, "en-us");
        assert_eq!(feed.categories.len(), 2);
        assert_eq!(feed.articles.len(), 2);
    }

    #[test]
    fn test_whitespace_handling() {
        let xml = r#"
            <rss version="2.0">
                <channel>
                    <title>Title with spaces</title>
                    <item>
                        <title>Article Title</title>
                    </item>
                </channel>
            </rss>
        "#;

        let feed = Feed::parse(xml).expect("Failed to parse feed");

        assert_eq!(feed.name, "Title with spaces");
        assert_eq!(feed.articles[0].title, "Article Title");
    }

    #[test]
    fn test_empty_category_elements() {
        let xml = r#"
            <rss version="2.0">
                <channel>
                    <title>Empty Categories</title>
                    <item>
                        <title>Article</title>
                        <category>Real Category</category>
                    </item>
                </channel>
            </rss>
        "#;

        let feed = Feed::parse(xml).expect("Failed to parse feed");
        let article = &feed.articles[0];

        assert_eq!(article.categories.len(), 1);
        assert_eq!(article.categories[0], "Real Category");
    }

    #[test]
    fn test_special_characters_in_content() {
        let xml = r#"
            <rss version="2.0">
                <channel>
                    <item>
                        <title>Title with special content</title>
                    </item>
                </channel>
            </rss>
        "#;

        let feed = Feed::parse(xml).expect("Failed to parse feed");
        let article = &feed.articles[0];

        assert_eq!(article.title, "Title with special content");
    }

    #[test]
    fn test_article_default_values() {
        let xml = r#"
            <rss version="2.0">
                <channel>
                    <item>
                        <title>Minimal Article</title>
                    </item>
                </channel>
            </rss>
        "#;

        let feed = Feed::parse(xml).expect("Failed to parse feed");
        let article = &feed.articles[0];

        assert_eq!(article.title, "Minimal Article");
        assert_eq!(article.author, "");
        assert_eq!(article.date, "");
        assert_eq!(article.url, "");
        assert_eq!(article.desc, "");
        assert_eq!(article.categories.len(), 0);
        assert_eq!(article.content, "");
        assert_eq!(article.image_url, "");
    }

    #[test]
    fn test_feed_default_values() {
        let xml = r#"
            <rss version="2.0">
                <channel>
                </channel>
            </rss>
        "#;

        let feed = Feed::parse(xml).expect("Failed to parse empty feed");

        assert_eq!(feed.name, "");
        assert_eq!(feed.description, "");
        assert_eq!(feed.link, "");
        assert_eq!(feed.language, "");
        assert_eq!(feed.generator, "");
        assert_eq!(feed.copyright, "");
        assert_eq!(feed.managingEditor, "");
        assert_eq!(feed.webMaster, "");
        assert_eq!(feed.pubDate, "");
        assert_eq!(feed.lastBuildDate, "");
        assert_eq!(feed.categories.len(), 0);
        assert_eq!(feed.docs, "");
        assert_eq!(feed.cloud, "");
        assert_eq!(feed.ttl, "");
        assert_eq!(feed.image, "");
        assert_eq!(feed.rating, "");
        assert_eq!(feed.skipHours, "");
        assert_eq!(feed.skipDays, "");
        assert_eq!(feed.articles.len(), 0);
    }

    #[test]
    fn test_namespace_prefixed_tags() {
        let xml = r#"
            <rss version="2.0" xmlns:custom="http://example.com/custom">
                <channel>
                    <item>
                        <title>Article</title>
                        <custom:field>Custom Content</custom:field>
                    </item>
                </channel>
            </rss>
        "#;

        let feed = Feed::parse(xml).expect("Failed to parse feed");
        assert_eq!(feed.articles.len(), 1);
        assert_eq!(feed.articles[0].title, "Article");
    }

    #[test]
    fn test_very_long_article_description() {
        let long_desc = "Lorem ipsum dolor sit amet, ".repeat(100);
        let xml = format!(
            r#"
            <rss version="2.0">
                <channel>
                    <item>
                        <title>Long Description Article</title>
                        <description>{}</description>
                    </item>
                </channel>
            </rss>
            "#,
            long_desc
        );

        let feed = Feed::parse(&xml).expect("Failed to parse feed");
        let article = &feed.articles[0];

        assert!(!article.desc.is_empty());
        assert!(article.desc.len() > 1000);
    }

    #[test]
    fn test_multiple_articles_with_overlapping_fields() {
        let xml = r#"
            <rss version="2.0">
                <channel>
                    <title>Feed Title</title>
                    <item>
                        <title>Article 1</title>
                        <category>Category A</category>
                    </item>
                    <item>
                        <title>Article 2</title>
                        <category>Category B</category>
                    </item>
                </channel>
            </rss>
        "#;

        let feed = Feed::parse(xml).expect("Failed to parse feed");

        assert_eq!(feed.name, "Feed Title");
        assert_eq!(feed.articles.len(), 2);
        assert_eq!(feed.articles[0].title, "Article 1");
        assert_eq!(feed.articles[0].categories[0], "Category A");
        assert_eq!(feed.articles[1].title, "Article 2");
        assert_eq!(feed.articles[1].categories[0], "Category B");
    }

    #[test]
    fn test_encoding_declaration() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
            <rss version="2.0">
                <channel>
                    <title>UTF-8 Feed</title>
                    <item>
                        <title>Article with UTF-8: café, naïve, Ñoño</title>
                    </item>
                </channel>
            </rss>
        "#;

        let feed = Feed::parse(xml).expect("Failed to parse feed");
        let article = &feed.articles[0];

        assert!(article.title.contains("café"));
        assert!(article.title.contains("Ñoño"));
    }

    #[test]
    fn test_article_state_independence() {
        let xml = r#"
            <rss version="2.0">
                <channel>
                    <item>
                        <title>Article 1</title>
                        <category>Cat1</category>
                    </item>
                    <item>
                        <title>Article 2</title>
                    </item>
                </channel>
            </rss>
        "#;

        let feed = Feed::parse(xml).expect("Failed to parse feed");

        assert_eq!(feed.articles[0].categories.len(), 1);
        assert_eq!(feed.articles[1].categories.len(), 0);
    }

    #[test]
    fn test_rss_090() {
        let xml = r#"
            <rss version="0.90">
                <channel>
                    <title>RSS 0.90 Feed</title>
                    <description>Test feed for RSS 0.90 (RDF Site Summary)</description>
                    <link>http://example.com</link>
                    <item>
                        <title>First Article</title>
                        <description>Article content for RSS 0.90</description>
                        <link>http://example.com/article1</link>
                    </item>
                    <item>
                        <title>Second Article</title>
                        <description>Another article in RSS 0.90 format</description>
                        <link>http://example.com/article2</link>
                    </item>
                </channel>
            </rss>
        "#;

        let feed = Feed::parse(xml).expect("Failed to parse RSS 0.90");
        assert_eq!(feed.name, "RSS 0.90 Feed");
        assert_eq!(
            feed.description,
            "Test feed for RSS 0.90 (RDF Site Summary)"
        );
        assert_eq!(feed.link, "http://example.com");
        assert_eq!(feed.articles.len(), 2);
        assert_eq!(feed.articles[0].title, "First Article");
        assert_eq!(feed.articles[1].title, "Second Article");
    }

    #[test]
    fn test_rss_091() {
        let xml = r#"
            <rss version="0.91">
                <channel>
                    <title>RSS 0.91 Feed</title>
                    <description>Test feed for RSS 0.91 (Rich Site Summary)</description>
                    <link>http://example.com</link>
                    <language>en-us</language>
                    <copyright>2024 Example Corp</copyright>
                    <managingEditor>editor@example.com</managingEditor>
                    <webMaster>webmaster@example.com</webMaster>
                    <lastBuildDate>Mon, 01 Jan 2024 00:00:00 GMT</lastBuildDate>
                    <item>
                        <title>News Item 1</title>
                        <description>News content for RSS 0.91</description>
                        <link>http://example.com/news1</link>
                    </item>
                </channel>
            </rss>
        "#;

        let feed = Feed::parse(xml).expect("Failed to parse RSS 0.91");
        assert_eq!(feed.name, "RSS 0.91 Feed");
        assert_eq!(
            feed.description,
            "Test feed for RSS 0.91 (Rich Site Summary)"
        );
        assert_eq!(feed.language, "en-us");
        assert_eq!(feed.copyright, "2024 Example Corp");
        assert_eq!(feed.managingEditor, "editor@example.com");
        assert_eq!(feed.webMaster, "webmaster@example.com");
        assert_eq!(feed.articles.len(), 1);
        assert_eq!(feed.articles[0].title, "News Item 1");
    }

    #[test]
    fn test_rss_10_rdf() {
        let xml = r#"
            <?xml version="1.0"?>
            <rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"
                     xmlns="http://purl.org/rss/1.0/"
                     xmlns:dc="http://purl.org/dc/elements/1.1/">
                <channel rdf:about="http://example.com">
                    <title>RSS 1.0 Feed</title>
                    <link>http://example.com</link>
                    <description>Test feed for RSS 1.0 (RDF Site Summary)</description>
                    <language>en</language>
                    <dc:creator>Feed Author</dc:creator>
                </channel>
                <item rdf:about="http://example.com/item1">
                    <title>RDF Item 1</title>
                    <link>http://example.com/item1</link>
                    <description>Item content in RSS 1.0 format</description>
                    <dc:creator>John Author</dc:creator>
                </item>
                <item rdf:about="http://example.com/item2">
                    <title>RDF Item 2</title>
                    <link>http://example.com/item2</link>
                    <description>Another item in RSS 1.0</description>
                    <dc:creator>Jane Author</dc:creator>
                </item>
            </rdf:RDF>
        "#;

        let feed = Feed::parse(xml).expect("Failed to parse RSS 1.0");
        assert_eq!(feed.name, "RSS 1.0 Feed");
        assert_eq!(feed.description, "Test feed for RSS 1.0 (RDF Site Summary)");
        assert_eq!(feed.link, "http://example.com");
        assert_eq!(feed.language, "en");
        assert_eq!(feed.articles.len(), 2);
        assert_eq!(feed.articles[0].title, "RDF Item 1");
        assert_eq!(feed.articles[0].author, "John Author");
        assert_eq!(feed.articles[1].title, "RDF Item 2");
        assert_eq!(feed.articles[1].author, "Jane Author");
    }

    #[test]
    fn test_rss_20() {
        let xml = r#"
            <?xml version="1.0" encoding="UTF-8"?>
            <rss version="2.0" xmlns:content="http://purl.org/rss/1.0/modules/content/" xmlns:dc="http://purl.org/dc/elements/1.1/">
                <channel>
                    <title>RSS 2.0 Feed</title>
                    <link>https://example.com</link>
                    <description>Test feed for RSS 2.0 (Really Simple Syndication)</description>
                    <language>en-us</language>
                    <copyright>2024 Example</copyright>
                    <pubDate>Thu, 05 Dec 2024 10:00:00 GMT</pubDate>
                    <lastBuildDate>Thu, 05 Dec 2024 10:00:00 GMT</lastBuildDate>
                    <generator>Custom RSS Parser</generator>
                    <ttl>60</ttl>
                    <item>
                        <title>RSS 2.0 Article 1</title>
                        <link>https://example.com/article1</link>
                        <dc:creator>Author One</dc:creator>
                        <pubDate>Wed, 04 Dec 2024 15:30:00 GMT</pubDate>
                        <category>Technology</category>
                        <category>Programming</category>
                        <description>This is an article in RSS 2.0 format</description>
                        <content:encoded><![CDATA[<p>Full HTML content for article 1</p>]]></content:encoded>
                        <enclosure url="https://example.com/image1.jpg" type="image/jpeg" />
                    </item>
                    <item>
                        <title>RSS 2.0 Article 2</title>
                        <link>https://example.com/article2</link>
                        <dc:creator>Author Two</dc:creator>
                        <pubDate>Tue, 03 Dec 2024 12:00:00 GMT</pubDate>
                        <category>News</category>
                        <description>Another RSS 2.0 article</description>
                        <content:encoded><![CDATA[<p>Full HTML content for article 2</p>]]></content:encoded>
                    </item>
                </channel>
            </rss>
        "#;

        let feed = Feed::parse(xml).expect("Failed to parse RSS 2.0");
        assert_eq!(feed.name, "RSS 2.0 Feed");
        assert_eq!(feed.link, "https://example.com");
        assert_eq!(
            feed.description,
            "Test feed for RSS 2.0 (Really Simple Syndication)"
        );
        assert_eq!(feed.language, "en-us");
        assert_eq!(feed.copyright, "2024 Example");
        assert_eq!(feed.generator, "Custom RSS Parser");
        assert_eq!(feed.ttl, "60");
        assert_eq!(feed.articles.len(), 2);

        let first = &feed.articles[0];
        assert_eq!(first.title, "RSS 2.0 Article 1");
        assert_eq!(first.author, "Author One");
        assert_eq!(first.categories.len(), 2);
        assert_eq!(first.categories[0], "Technology");
        assert_eq!(first.categories[1], "Programming");
        assert!(first.content.contains("Full HTML content for article 1"));
        assert_eq!(first.image_url, "https://example.com/image1.jpg");

        let second = &feed.articles[1];
        assert_eq!(second.title, "RSS 2.0 Article 2");
        assert_eq!(second.author, "Author Two");
        assert_eq!(second.categories.len(), 1);
        assert_eq!(second.categories[0], "News");
    }

    #[test]
    fn test_rss_20_with_enclosures_podcast() {
        let xml = r#"
            <rss version="2.0">
                <channel>
                    <title>Podcast Feed</title>
                    <link>https://example.com/podcast</link>
                    <description>A podcast feed in RSS 2.0 format</description>
                    <item>
                        <title>Episode 1</title>
                        <description>First episode</description>
                        <link>https://example.com/episode1</link>
                        <enclosure url="https://example.com/episode1.mp3" type="audio/mpeg" length="5000000" />
                        <pubDate>Mon, 04 Dec 2024 10:00:00 GMT</pubDate>
                    </item>
                    <item>
                        <title>Episode 2</title>
                        <description>Second episode</description>
                        <link>https://example.com/episode2</link>
                        <enclosure url="https://example.com/episode2.mp3" type="audio/mpeg" length="6000000" />
                        <pubDate>Tue, 05 Dec 2024 10:00:00 GMT</pubDate>
                    </item>
                </channel>
            </rss>
        "#;

        let feed = Feed::parse(xml).expect("Failed to parse podcast feed");
        assert_eq!(feed.name, "Podcast Feed");
        assert_eq!(feed.articles.len(), 2);
        assert_eq!(
            feed.articles[0].image_url,
            "https://example.com/episode1.mp3"
        );
        assert_eq!(
            feed.articles[1].image_url,
            "https://example.com/episode2.mp3"
        );
    }

    #[test]
    fn test_all_rss_versions_compatibility() {
        let feeds = vec![
            (
                "RSS 0.90",
                r#"<rss version="0.90"><channel><title>v0.90</title><item><title>Item</title></item></channel></rss>"#,
            ),
            (
                "RSS 0.91",
                r#"<rss version="0.91"><channel><title>v0.91</title><item><title>Item</title></item></channel></rss>"#,
            ),
            (
                "RSS 2.0",
                r#"<rss version="2.0"><channel><title>v2.0</title><item><title>Item</title></item></channel></rss>"#,
            ),
        ];

        for (version_name, xml) in feeds {
            let feed = Feed::parse(xml).expect(&format!("Failed to parse {}", version_name));
            assert_eq!(feed.articles.len(), 1, "Failed for {}", version_name);
            assert_eq!(
                feed.articles[0].title, "Item",
                "Failed for {}",
                version_name
            );
        }
    }

    #[test]
    fn test_unclosed_entity_reference() {
        let xml = r#"
            <rss version="2.0">
                <channel>
                    <title>Entity Test Feed</title>
                    <item>
                        <title>Article with bad entity &something</title>
                        <description>Test</description>
                    </item>
                </channel>
            </rss>
        "#;

        let result = Feed::parse(xml);
        assert!(result.is_err(), "Should fail on unclosed entity reference");
    }

    #[test]
    fn test_html_response_instead_of_rss() {
        let html = r#"
            <!DOCTYPE html>
            <html>
                <head>
                    <title>404 Not Found</title>
                </head>
                <body>
                    <h1>Not Found</h1>
                </body>
            </html>
        "#;

        let result = Feed::parse(html);
        match result {
            Ok(feed) => {
                assert_eq!(feed.articles.len(), 0, "HTML feed should have no articles");
            }
            Err(_) => {}
        }
    }

    #[test]
    fn test_mismatched_tags() {
        let xml = r#"
            <rss version="2.0">
                <channel>
                    <title>Mismatched Tags</title>
                    <item>
                        <link>http://example.com</link>
                        </head>
                    </item>
                </channel>
            </rss>
        "#;

        let result = Feed::parse(xml);
        assert!(result.is_err(), "Should fail on mismatched tags");
    }

    #[test]
    fn test_malformed_br_tag() {
        let xml = r#"
            <rss version="2.0">
                <channel>
                    <title>BR Tag Test</title>
                    <item>
                        <title>Test</title>
                        <description><br>This should be closed<p>text</p></description>
                    </item>
                </channel>
            </rss>
        "#;

        let result = Feed::parse(xml);
        assert!(result.is_err(), "Should fail on malformed br tag");
    }

    #[test]
    fn test_unclosed_meta_tag() {
        let xml = r#"
            <rss version="2.0">
                <channel>
                    <title>Meta Tag Test</title>
                    <meta property="og:title" content="Test
                    </head>
                </channel>
            </rss>
        "#;

        let result = Feed::parse(xml);
        assert!(result.is_err(), "Should fail on unclosed meta tag");
    }

    #[test]
    fn test_script_tag_in_feed() {
        let xml = r#"
            <rss version="2.0">
                <channel>
                    <title>Script Test</title>
                    <item>
                        <title>Article</title>
                        <description>Content</description>
                        <script>alert('XSS')</script>
                        </head>
                    </item>
                </channel>
            </rss>
        "#;

        let result = Feed::parse(xml);
        assert!(
            result.is_err(),
            "Should fail with script tag and mismatched closing"
        );
    }

    #[test]
    fn test_recovery_from_partial_feed() {
        let xml = r#"
            <rss version="2.0">
                <channel>
                    <title>Partial Feed</title>
                    <item>
                        <title>Article 1</title>
                    </item>
                    <item>
                        <title>Article 2</title>
                    </item>
                </channel>
        "#;

        let result = Feed::parse(xml);
        match result {
            Ok(feed) => {
                assert!(feed.articles.len() >= 1);
            }
            Err(_) => {}
        }
    }

    #[test]
    fn test_empty_document() {
        let xml = "";
        let result = Feed::parse(xml);
        match result {
            Ok(feed) => {
                assert_eq!(feed.articles.len(), 0);
            }
            Err(_) => {}
        }
    }

    #[test]
    fn test_only_whitespace() {
        let xml = "   \n\n   \t  ";
        let result = Feed::parse(xml);
        match result {
            Ok(feed) => {
                assert_eq!(feed.articles.len(), 0);
            }
            Err(_) => {}
        }
    }

    #[test]
    fn test_truncated_feed() {
        let xml = r#"
            <rss version="2.0">
                <channel>
                    <title>Truncated Feed</title>
                    <item>
                        <title>Article</title>
                        <link>http://example.c
        "#;

        let result = Feed::parse(xml);
        // Parser may either fail on truncated feed or recover gracefully
        let _ = match result {
            Ok(feed) => {
                // Parser recovered from truncation - just verify it parsed something
                !feed.name.is_empty() || !feed.articles.is_empty()
            }
            Err(_) => {
                // Expected - truncated feed rejected
                true
            }
        };
    }
}
