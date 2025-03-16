use anyhow::{Context, Result};
use media_list::{MediaItem, get_media_list};
use scraper::{ElementRef, Html, Selector};
use serde::{Deserialize, Serialize};
use std::{fs, hash::{DefaultHasher, Hash, Hasher}};
use url::Url;
mod headers;
mod media_list;

use headers::get_headers;

#[derive(Serialize, Deserialize, Hash)]
struct NewsItem {
    hash: String,
    title: String,
    link: String,
}

impl NewsItem {
    fn new(title: String, link: String) -> Self {
        let mut hasher = DefaultHasher::new();
        title.hash(&mut hasher);
        link.hash(&mut hasher);
        let hash = format!("{:x}", hasher.finish()); // Хеш в виде строки (hex)

        Self { title, link, hash }
    }
}

fn get_title(el: ElementRef, sel: &str) -> Option<String> {
    Selector::parse(sel)
        .ok()
        .and_then(|selector| el.select(&selector).next())
        .and_then(|item| item.text().next())
        .map(|text| text.trim().to_string())
}

fn get_link(news_el: ElementRef, base_url: &str) -> Option<String> {
    let href = news_el
        .select(&Selector::parse("a").ok()?)
        .next()?
        .attr("href")?
        .trim();

    if href.is_empty() {
        return None;
    }

    Url::parse(href)
        .or_else(|_| Url::parse(base_url).and_then(|base| base.join(href)))
        .ok()
        .filter(|url| url.as_str() != base_url)
        .map(|url| url.to_string())
}

fn parse(content: &str, media: &MediaItem) -> Result<()> {
    let document = Html::parse_document(content);
    let container_selector = Selector::parse(&media.container_selector).unwrap();
    let item_selector = Selector::parse(&media.item_selector).unwrap();
    let container = document.select(&container_selector).next();

    match container {
        Some(container) => {
            let news_items = container.select(&item_selector);

            let parsed: Vec<NewsItem> = news_items
                .filter_map(|item| {
                    let title = get_title(item, &media.title_selector)?;
                    let link = get_link(item, &media.url)?;
                    Some(NewsItem::new(title, link))
                })
                .collect();

            if !parsed.is_empty() {
                let serialized = serde_json::to_string_pretty(&parsed)?;
                fs::write(format!("./json/{}.json", &media.name), serialized)?;
            }
        }
        None => {
            println!("{} - container not found, continue...", &media.name);
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let media_list = get_media_list()
        .unwrap()
        .into_iter()
        .filter(|x| !x.url.is_empty());

    let client = reqwest::Client::new();

    if !fs::metadata("./json").is_ok() {
        fs::create_dir("./json").context("Create JSON directory error")?;
    }

    for media in media_list {
        let headers = get_headers()?;
        let content = client.get(&media.url).headers(headers).send().await;

        match content {
            Ok(resp) => {
                let content = resp.text().await.unwrap();
                let res = parse(&content, &media);

                if res.is_ok() {
                    println!("[*] Parsing done {}", &media.name);
                }
            }
            _ => {
                println!("{} - {} Request error", &media.name, &media.url);
            }
        }
    }

    Ok(())
}
