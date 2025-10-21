use anyhow::Result;
use rand::seq::SliceRandom;
use std::fmt::Write;

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
impl OutputTrait for () {}

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

impl UsecaseTrait<(), ()> for RandomRecommendUsecase {
    fn new(_: ()) -> Self {
        Self {
            repository: Repository::new(),
        }
    }
    async fn handle(&self) -> Result<()> {
        let pick_random = |mut items: Vec<Item>| {
            items.shuffle(&mut rand::rng());
            items.into_iter().take(5).collect::<Vec<_>>()
        };

        let qiita_picks = pick_random(self.repository.qiita.fetch_items(1).await?);
        let zenn_picks = pick_random(self.repository.zenn.fetch_items(1).await?);

        let notifier = LineNotificationRepository::new()?;

        let make_msg = |title: &str, picks: &[Item]| {
            picks
                .iter()
                .enumerate()
                .map(|(i, it)| format!("{}. {} {}\n", i + 1, it.title, it.url))
                .fold(
                    format!("{} 今日のランダムチョイス\n", title),
                    |mut acc, line| {
                        let _ = write!(acc, "{}", line);
                        acc
                    },
                )
        };

        let qiita_msg = make_msg("Qiita", &qiita_picks);
        let zenn_msg = make_msg("Zenn", &zenn_picks);

        notifier.send(&qiita_msg).await?;
        notifier.send(&zenn_msg).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn should_fetch_articles_return_qiita_and_zenn_picks() {
        let usecase = RandomRecommendUsecase::new(());
        if let Err(e) = usecase.handle().await {
            eprintln!("Usecase execution failed: {e:?}");
            panic!("failed to execute usecase");
        }
    }
}
