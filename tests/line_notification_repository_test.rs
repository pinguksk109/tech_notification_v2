use std::env;
use tech_notification_v2::{
    application::port::notification_port::NotificationPort,
    infrastructure::line_notification_repository::LineNotificationRepository,
};

// 手動実行用。誤爆防止に ignore を付けています。
// 実行: cargo test --test line_notification_repository_test -- --ignored --nocapture
#[tokio::test]
#[ignore]
async fn should_send_line_notification_return_ok() {
    env::set_var("LINE_USER_ID", "Uxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");
    env::set_var(
        "LINE_BEARER_TOKEN",
        "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
    );

    let repo =
        LineNotificationRepository::new().expect("failed to init LineNotificationRepository");
    let message = format!("疎通テスト from Rust");
    if let Err(e) = repo.send(&message).await {
        eprintln!("❌ LINE通知に失敗しました: {}", e);
        panic!("LINE送信失敗");
    }
}
