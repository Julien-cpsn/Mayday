use crate::models::driver::MessagingDriver;
use crate::models::driver_config::DriverConfig;
use crate::models::message::Message;
use async_trait::async_trait;
use chrono::Local;
use ratatui::prelude::Color;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use ormlite::Model;
use ormlite::sqlite::SqliteConnection;
use tokio::time::sleep;

const SENDER: &str = "Yourself";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LoopbackConfig;

impl DriverConfig for LoopbackConfig {
}

#[derive(Clone)]
pub struct LoopbackMessaging {
    config: LoopbackConfig,
}

#[async_trait]
impl MessagingDriver for LoopbackMessaging {
    type Config = LoopbackConfig;

    async fn new(config: LoopbackConfig) -> anyhow::Result<Self> {
        Ok(LoopbackMessaging {
            config
        })
    }

    fn config(&self) -> &LoopbackConfig {
        &self.config
    }

    fn name(&self) -> &str {
        "Loopback"
    }

    fn icon(&self) -> &str {
        "✉ "
    }

    fn color(&self) -> Color {
        Color::Blue
    }

    async fn send_message(&mut self, _message: &Message) -> anyhow::Result<()> {
        Ok(())
    }

    async fn poll_received_messages(&mut self, db: &mut SqliteConnection) -> anyhow::Result<()> {
        sleep(Duration::from_secs(10)).await;
        
        Message {
            sender: Some(String::from(SENDER)),
            text: String::from("T'es là mec ?"),
            timestamp: Local::now(),
        }
            .insert(db);
        
        Ok(())
    }
}