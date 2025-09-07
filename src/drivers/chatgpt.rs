use crate::models::driver::MessagingDriver;
use crate::models::driver_config::DriverConfig;
use crate::models::message::Message;
use async_trait::async_trait;
use ormlite::sqlite::SqliteConnection;
use parking_lot::Mutex;
use ratatui::prelude::Color;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use chrono::Local;
use futures::future::BoxFuture;
use futures::FutureExt;
use log::trace;
use ormlite::Model;
use reqwest::{Client, Response};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChatGptConfig {
    pub api_key: String,
    pub temperature: f32,
}

impl DriverConfig for ChatGptConfig {
}

pub struct ChatGptMessaging<'a> {
    config: ChatGptConfig,
    tx: Sender<BoxFuture<'a, Result<Response, reqwest::Error>>>,
    rx: Arc<Mutex<Receiver<BoxFuture<'a, Result<Response, reqwest::Error>>>>>,
}

#[derive(Serialize)]
struct ChatGptMessage {
    pub role: String,
    pub content: String
}

#[async_trait]
impl MessagingDriver for ChatGptMessaging<'_> {
    type Config = ChatGptConfig;

    async fn new(config: ChatGptConfig) -> anyhow::Result<Self> {
        let (tx, rx) = channel();
        Ok(ChatGptMessaging {
            config,
            tx,
            rx: Arc::new(Mutex::new(rx)),
        })
    }

    fn config(&self) -> &Self::Config {
        &self.config
    }

    fn name(&self) -> &str {
        "ChatGPT"
    }

    fn icon(&self) -> &str {
        "֍"
    }

    fn color(&self) -> Color {
        Color::Green
    }

    async fn send_message(&mut self, db: &mut SqliteConnection, message: &Message) -> anyhow::Result<()> {
        let api_key = self.config.api_key.clone();
        let temperature = self.config.temperature;
        let tx = self.tx.clone();

        let mut messages = vec![
            ChatGptMessage {
                role: String::from("system"),
                content: String::from("You are a helpful assistant")
            },
        ];

        for message in Message::select().fetch_all(db).await? {
            messages.push(ChatGptMessage {
                role: match message.sender {
                    None => String::from("user"),
                    Some(_) => String::from("system"),
                },
                content: message.text,
            })
        }

        messages.push(ChatGptMessage {
            role: String::from("user"),
            content: message.text.clone(),
        });

        // Construct payload in OpenAI chat format
        let body = json!({
            "model": "gpt-4o-mini",
            "temperature": temperature,
            "messages": messages,
        });

        let response = Client::new()
            .post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(&api_key)
            .json(&body)
            .send();

        tx.send(response.boxed()).ok();

        Ok(())
    }

    async fn active_poll_received_messages(&mut self, db: &mut SqliteConnection) -> anyhow::Result<()> {
        let rx = self.rx.lock();

        while let Ok(stream) = rx.try_recv() {
            trace!("Message received");

            let response = stream.await?;

            if let Ok(json_response) = response.json::<serde_json::Value>().await {
                let response = json_response["choices"][0]["message"]["content"].as_str().unwrap().to_string();

                Message {
                    timestamp: Local::now(),
                    sender: Some(String::from("ChatGPT")),
                    text: response,
                }
                    .insert(&mut *db)
                    .await?;
            }
        }

        Ok(())
    }

    async fn passive_poll_received_messages(&mut self, _: &mut SqliteConnection) -> anyhow::Result<()> {
        Ok(())
    }
}