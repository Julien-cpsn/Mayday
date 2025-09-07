use crate::models::driver::MessagingDriver;
use crate::models::driver_config::DriverConfig;
use crate::models::message::Message;
use async_trait::async_trait;
use chrono::Local;
use ratatui::prelude::Color;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use log::{trace};
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
        "⟳"
    }

    fn color(&self) -> Color {
        Color::Blue
    }

    async fn send_message(&mut self, _: &mut SqliteConnection, _: &Message) -> anyhow::Result<()> {
        Ok(())
    }

    async fn active_poll_received_messages(&mut self, _: &mut SqliteConnection) -> anyhow::Result<()> {
        Ok(())
    }

    async fn passive_poll_received_messages(&mut self, db: &mut SqliteConnection) -> anyhow::Result<()> {
        loop {
            sleep(Duration::from_secs(10)).await;

            trace!("New loopback messages received");

            Message {
                sender: Some(String::from(SENDER)),
                text: String::from("You here?"),
                timestamp: Local::now(),
            }
                .insert(&mut *db)
                .await?;
        }
    }
}