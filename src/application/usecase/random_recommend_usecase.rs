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
