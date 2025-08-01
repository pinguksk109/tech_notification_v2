use std::error::Error;
use serde_json::Value;

use async_trait::async_trait;
use reqwest::{Client, StatusCode};

use crate::application::{domain::model::item::Item, port::article_port::ArticlePort};

pub struct ZennArticleRepository {
    client: Client,
}

impl ZennArticleRepository {
    pub fn new() -> Self {
        ZennArticleRepository {
            client: Client::new(),
        }
    }
}

#[async_trait]
impl ArticlePort for ZennArticleRepository {
    async fn fetch_items(&self, page: usize) -> Result<Vec<Item>, Box<dyn Error + Send + Sync>> {
        let url = format!("https://zenn.dev/api/articles?order=latest&page={}", page);
        let resp = self.client.get(&url).send().await?;
        if resp.status() != StatusCode::OK {
            return Err(format!("Zenn API error {}", resp.status().as_u16()).into());
        }

        let body: Value = resp.json().await?;
        let raw_articles = body.get("articles")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        let items: Vec<Item> = raw_articles
            .into_iter()
            .map(|art| {
                let title = art.get("title").and_then(Value::as_str).unwrap_or("").to_string();
                let path = art.get("path").and_then(Value::as_str).unwrap_or("");
                let likes_count = art.get("liked_count").and_then(Value::as_u64).unwrap_or(0) as u32;
                Item {
                    title,
                    url: format!("https://zenn.dev{}", path),
                    likes_count,
                } 
            })
            .collect();

        Ok(items)
    }
}