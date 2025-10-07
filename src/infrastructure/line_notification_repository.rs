use anyhow::{Context, Result, anyhow};
use async_trait::async_trait;
use reqwest::{Client, StatusCode};
use serde::Serialize;
use std::env;

use crate::application::port::notification_port::NotificationPort;

#[derive(Debug)]
pub struct LineNotificationRepository {
    to: String,
    bearer_token: String,
    client: Client,
}

impl LineNotificationRepository {
    pub fn new() -> Result<Self> {
        let to = env::var("LINE_USER_ID").context("環境変数 LINE_USER_ID が設定されていません")?;
        let bearer_token = env::var("LINE_BEARER_TOKEN")
            .context("環境変数 LINE_BEARER_TOKEN が設定されていません")?;

        Ok(Self {
            to,
            bearer_token,
            client: Client::new(),
        })
    }
}

#[derive(Serialize)]
struct LineMessagePayload<'a> {
    to: &'a str,
    messages: Vec<LineMessage<'a>>,
}

#[derive(Serialize)]
struct LineMessage<'a> {
    #[serde(rename = "type")]
    msg_type: &'a str,
    text: &'a str,
}

#[async_trait]
impl NotificationPort for LineNotificationRepository {
    async fn send(&self, message: &str) -> Result<()> {
        let payload = LineMessagePayload {
            to: &self.to,
            messages: vec![LineMessage {
                msg_type: "text",
                text: message,
            }],
        };

        let resp = self
            .client
            .post("https://api.line.me/v2/bot/message/push")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.bearer_token))
            .json(&payload)
            .send()
            .await
            .context("LINE APIリクエストに失敗しました")?;

        if resp.status() != StatusCode::OK {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(anyhow!(
                "LINE APIがエラーを返しました。status: {}, body: {}, message: {}",
                status,
                body,
                message
            ));
        }

        Ok(())
    }
}
