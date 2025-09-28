use anyhow::Result;
use async_trait::async_trait;
use rand::seq::SliceRandom;

use crate::application::{
    base::{OutputTrait, UsecaseTrait},
    domain::model::item::Item,
    port::article_port::ArticlePort,
};
use crate::infrastructure::{
    qiita_article_repository::QiitaArticleRepository,
    zenn_article_repository::ZennArticleRepository,
};

#[derive(Debug)]
pub struct RecommendOutput {
    pub qiita: Vec<Item>,
    pub zenn: Vec<Item>,
}

impl OutputTrait for RecommendOutput {}

#[derive(Debug)]
pub struct Repository<Q, Z> {
    qiita: Q,
    zenn: Z,
}

impl Repository<QiitaArticleRepository, ZennArticleRepository> {
    fn new() -> Self {
        Self {
            qiita: QiitaArticleRepository::new(),
            zenn: ZennArticleRepository::new(),
        }
    }
}

#[derive(Debug)]
pub struct RandomRecommendUsecase {
    repository: Repository<QiitaArticleRepository, ZennArticleRepository>,
}

#[async_trait]
impl UsecaseTrait<(), RecommendOutput> for RandomRecommendUsecase {
    fn new(_: ()) -> Self {
        Self {
            repository: Repository::new(),
        }
    }
    async fn handle(&self) -> Result<RecommendOutput> {
        let mut rng = rand::rng();

        let mut qiita_items = self.repository.qiita.fetch_items(1).await?;
        qiita_items.shuffle(&mut rng);
        let qiita_picks = qiita_items.into_iter().take(5).collect::<Vec<_>>();

        let mut zenn_items: Vec<Item> = self.repository.zenn.fetch_items(1).await?;
        zenn_items.shuffle(&mut rng);
        let zenn_picks = zenn_items.into_iter().take(5).collect::<Vec<_>>();

        Ok(RecommendOutput {
            qiita: qiita_picks,
            zenn: zenn_picks,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn should_fetch_articles_return_qiita_and_zenn_picks() {
        // 1. setup
        let usecase = RandomRecommendUsecase::new(());

        // 2. execute
        let actual = usecase.handle().await;

        // 3. assert
        match actual {
            Ok(output) => {
                println!("--- Qiita (picked {}) ---", output.qiita.len());
                for (i, item) in output.qiita.iter().enumerate() {
                    println!(
                        "{}. {} [{}] ❤️{}",
                        i + 1,
                        item.title,
                        item.url,
                        item.likes_count
                    );
                }

                println!("--- Zenn (picked {}) ---", output.zenn.len());
                for (i, item) in output.zenn.iter().enumerate() {
                    println!(
                        "{}. {} [{}] ❤️{}",
                        i + 1,
                        item.title,
                        item.url,
                        item.likes_count
                    );
                }
            }
            Err(e) => {
                eprintln!("Usecase execution failed: {:?}", e);
                panic!("failed to execute usecase");
            }
        }
    }
}
