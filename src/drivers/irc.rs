use crate::models::driver::MessagingDriver;
use crate::models::driver_config::DriverConfig;
use crate::models::message::Message;
use async_trait::async_trait;
use chrono::Local;
use futures::StreamExt;
use irc::client::prelude::Config;
use irc::client::Client;
use irc::proto::Command;
use log::{trace};
use ormlite::Model;
use ormlite::sqlite::SqliteConnection;
use ratatui::prelude::Color;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IrcConfig {
    pub channel: String,

    #[serde(flatten)]
    pub config: Config,
}

impl DriverConfig for IrcConfig {
}

pub struct IrcMessaging {
    config: IrcConfig,
    client: Client,
}


#[async_trait]
impl MessagingDriver for IrcMessaging {
    type Config = IrcConfig;

    async fn new(mut irc_config: IrcConfig) -> anyhow::Result<Self> {
        irc_config.config.channels = vec![irc_config.channel.clone()];
        let client = Client::from_config(irc_config.config.clone()).await?;

        client.identify()?;

        Ok(IrcMessaging {
            config: irc_config,
            client
        })
    }

    fn config(&self) -> &IrcConfig {
        &self.config
    }

    fn name(&self) -> &str {
        "IRC"
    }

    fn icon(&self) -> &str {
        "IRC"
    }

    fn color(&self) -> Color {
        Color::Blue
    }

    async fn send_message(&mut self, _: &mut SqliteConnection, message: &Message) -> anyhow::Result<()> {
        self.client.send_privmsg(&self.config.channel, &message.text)?;
        Ok(())
    }

    async fn active_poll_received_messages(&mut self, _: &mut SqliteConnection) -> anyhow::Result<()> {
        Ok(())
    }
    
    async fn passive_poll_received_messages(&mut self, db: &mut SqliteConnection) -> anyhow::Result<()> {
        let mut stream = self.client.stream()?;

        while let Some(message) = stream.next().await.transpose()? {
            let nickname = message.source_nickname().unwrap_or("unknown").to_string();

            if let Command::PRIVMSG(channel, text) = message.command {
                trace!("Message received in {channel}");
                
                if &text == "VERSION" {
                    continue;
                }

                Message {
                    timestamp: Local::now(),
                    sender: Some(nickname),
                    text,
                }
                    .insert(&mut *db)
                    .await?;
            }
        }
        
        Ok(())
    }
}