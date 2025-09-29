use crate::application::domain::model::item::Item;
use crate::application::port::article_port::ArticlePort;
use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::{Client, StatusCode};

#[derive(Debug)]
pub struct QiitaArticleRepository {
    client: Client,
}

impl QiitaArticleRepository {
    pub fn new() -> Self {
        QiitaArticleRepository {
            client: Client::new(),
        }
    }
}

#[async_trait]
impl ArticlePort for QiitaArticleRepository {
    async fn fetch_items(&self, page: usize) -> Result<Vec<Item>> {
        let url = format!("https://qiita.com/api/v2/items?page={}&per_page=100", page);
        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .with_context(|| format!("Qiita request failed: {}", url))?;
        let status = resp.status();

        if status == StatusCode::FORBIDDEN {
            anyhow::bail!("Qiita API: rate limited");
        }
        if status != StatusCode::OK {
            anyhow::bail!("Qiita API error {}", status.as_u16());
        }
        let items: Vec<Item> = resp.json().await.context("Qiita: failed to parse JSON")?;

        Ok(items)
    }
}
