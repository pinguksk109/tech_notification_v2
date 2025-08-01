use tech_notification_v2::{application::port::article_port::ArticlePort, infrastructure::qiita_article_repository::QiitaArticleRepository};

#[tokio::test]
async fn test_fetch_items_from_qiita() {
    let repository = QiitaArticleRepository::new();

    let actual = repository.fetch_items(1).await;

    match actual {
        Ok(items) => {
            println!("{:?}", items);
        }
        Err(e) => {
            panic!("API呼び出しに失敗しました: {}", e);
        }
    }
}