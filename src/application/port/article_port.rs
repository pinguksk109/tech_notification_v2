use crate::application::domain::model::item::Item;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait ArticlePort {
    const TARGET_PAGE_COUNT: usize = 10;

    async fn fetch_items(&self, page: usize) -> Result<Vec<Item>>;
}
