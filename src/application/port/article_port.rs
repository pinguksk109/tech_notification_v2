use async_trait::async_trait;
use crate::application::domain::model::item::Item;

#[async_trait]
pub trait ArticlePort {
    const TARGET_PAGE_COUNT: usize = 10;

    async fn fetch_items(
        &self,
        page: usize,
    ) -> Result<Vec<Item>, Box<dyn std::error::Error + Send + Sync>>;
}