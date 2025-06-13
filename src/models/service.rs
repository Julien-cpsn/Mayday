use crate::models::driver::ErasedMessagingDriver;
use crate::models::message::Message;
use chrono::Local;
use ormlite::sqlite::SqliteConnection;
use ormlite::Model;
use regex::Regex;
use uuid::Uuid;

pub struct  MessagingService {
    pub uuid: Uuid,
    pub discussion_name: String,
    pub tmp_messages: Vec<Message>,
    pub db: SqliteConnection,
    pub driver: Box<dyn ErasedMessagingDriver>
}

impl MessagingService {
    pub async fn try_load_messages(&mut self) -> anyhow::Result<bool> {
        let messages = Message::select().fetch_all(&mut self.db).await?;
        
        if self.tmp_messages.len() == messages.len() {
            Ok(false)
        }
        else {
            self.tmp_messages = messages;
            Ok(true)
        }
    }

    pub async fn send_message(&mut self, text: String) -> anyhow::Result<()> {
        let text = text.trim();

        let regex = Regex::new("[\r\n]+")?;
        let text = regex.replace_all(text, "\n").to_string();

        let message = Message {
            sender: None,
            text,
            timestamp: Local::now(),
        };

        message.clone().insert(&mut self.db).await?;
        self.driver.send_message(&message)?;
        self.tmp_messages.push(message.clone());

        Ok(())
    }

    pub async fn poll_received_messages(&mut self) -> anyhow::Result<bool> {
        let messages = self.driver.poll_received_messages()?;

        let has_received_messaged = !messages.is_empty();

        for message in messages {
            message.insert(&mut self.db).await?;
        }

        Ok(has_received_messaged)
    }
}