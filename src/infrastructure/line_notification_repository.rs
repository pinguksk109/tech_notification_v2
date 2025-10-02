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
