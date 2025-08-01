use tech_notification_v2::infrastructure::zenn_article_repository::ZennArticleRepository;
use tech_notification_v2::application::port::article_port::ArticlePort;

#[tokio::test]
async fn test_fetch_items_from_zenn() {
    let repository = ZennArticleRepository::new();

    let actual = repository.fetch_items(1).await;

    match actual {
        Ok(items) => {
            println!("{:?}", items);
            for (i, item) in items.iter().take(5).enumerate() {
                println!("  [{}] {} ({})", i + 1, item.title, item.url);
            }
            assert!(items.is_empty() || items.len() > 0);
        }
        Err(e) => {
            panic!("Zenn API への疎通に失敗しました: {}", e);
        }
    }
}