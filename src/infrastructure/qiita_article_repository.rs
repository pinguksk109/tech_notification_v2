use async_trait::async_trait;
use reqwest::{Client, StatusCode};
use crate::application::port::article_port::ArticlePort;
use crate::application::domain::model::item::Item;
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
    async fn fetch_items(
        &self,
        page: usize,
    ) -> Result<Vec<Item>, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!(
            "https://qiita.com/api/v2/items?page={}&per_page=100",
            page
        );
        let resp = self.client.get(&url).send().await?;
        let status = resp.status();

        if status == StatusCode::FORBIDDEN {
            return Err("Qiita API: rate limited".into());
        }
        if status != StatusCode::OK {
            return Err(format!("Qiita API error{}", status.as_u16()).into());
        }

        let items: Vec<Item> = resp.json().await?;

        Ok(items)
    }
}