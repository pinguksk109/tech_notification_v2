use anyhow::Result;
use rand::seq::SliceRandom;

use crate::infrastructure::{
    qiita_article_repository::QiitaArticleRepository,
    zenn_article_repository::ZennArticleRepository,
};
use crate::{
    application::{
        base::{InputTrait, OutputTrait, UsecaseTrait},
        domain::model::item::Item,
        port::{article_port::ArticlePort, notification_port::NotificationPort},
    },
    infrastructure::line_notification_repository::LineNotificationRepository,
};

impl InputTrait for () {}

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

impl UsecaseTrait<(), RecommendOutput> for RandomRecommendUsecase {
    fn new(_: ()) -> Self {
        Self {
            repository: Repository::new(),
        }
    }
    async fn handle(&self) -> Result<RecommendOutput> {
        let mut qiita_items = self.repository.qiita.fetch_items(1).await?;
        qiita_items.shuffle(&mut rand::rng());
        let qiita_picks = qiita_items.into_iter().take(5).collect::<Vec<_>>();

        let mut zenn_items: Vec<Item> = self.repository.zenn.fetch_items(1).await?;
        zenn_items.shuffle(&mut rand::rng());
        let zenn_picks = zenn_items.into_iter().take(5).collect::<Vec<_>>();

        let notifier = LineNotificationRepository::new()?;

        let qiita_msg = {
            let mut s = String::from("Qiita 今日のランダムチョイス\n");
            for (i, it) in qiita_picks.iter().enumerate() {
                let _ = std::fmt::Write::write_fmt(
                    &mut s,
                    format_args!("{}. {} {} ❤️{}\n", i + 1, it.title, it.url, it.likes_count),
                );
            }
            s
        };

        let zenn_msg = {
            let mut s = String::from("Zenn 今日のランダムチョイス\n");
            for (i, it) in qiita_picks.iter().enumerate() {
                let _ = std::fmt::Write::write_fmt(
                    &mut s,
                    format_args!("{}. {} {} ❤️{}\n", i + 1, it.title, it.url, it.likes_count),
                );
            }
            s
        };

        notifier.send(&qiita_msg).await?;
        notifier.send(&zenn_msg).await?;

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
                        "{}. {} [{}] {}",
                        i + 1,
                        item.title,
                        item.url,
                        item.likes_count
                    );
                }

                println!("--- Zenn (picked {}) ---", output.zenn.len());
                for (i, item) in output.zenn.iter().enumerate() {
                    println!(
                        "{}. {} [{}] {}",
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
