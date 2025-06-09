use crate::models::driver_config::DriverConfig;
use crate::models::driver::MessagingDriver;
use crate::models::message::{DateTime, Message};
use chrono::Local;
use ratatui::prelude::Color;
use serde::{Deserialize, Serialize};

const SENDER: &str = "Yourself";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LoopbackConfig;

impl DriverConfig for LoopbackConfig {
}

pub struct LoopbackMessaging {
    config: LoopbackConfig,
}

impl MessagingDriver for LoopbackMessaging {
    type Config = LoopbackConfig;

    fn new(config: LoopbackConfig) -> Self {
        LoopbackMessaging {
            config
        }
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

    fn send_message(&mut self, _message: &Message) -> anyhow::Result<()> {
        Ok(())
    }

    fn poll_received_messages(&mut self) -> anyhow::Result<Vec<Message>> {
        Ok(vec![
            Message {
                sender: Some(String::from(SENDER)),
                text: String::from("T'es là mec ?"),
                timestamp: DateTime(Local::now()),
            }
        ])
    }
}