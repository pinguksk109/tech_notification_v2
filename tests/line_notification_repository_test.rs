use tech_notification_v2::{
    application::port::notification_port::NotificationPort,
    infrastructure::line_notification_repository::LineNotificationRepository,
};

#[tokio::test]
#[ignore]
async fn should_send_line_notification_return_ok() {
    let repo =
        LineNotificationRepository::new().expect("failed to init LineNotificationRepository");
    let message = format!("疎通テスト from Rust");
    if let Err(e) = repo.send(&message).await {
        eprintln!("❌ LINE通知に失敗しました: {}", e);
        panic!("LINE送信失敗");
    }
}
