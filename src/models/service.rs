use crate::models::driver::ErasedMessagingDriver;
use crate::models::message::{DateTime, Message};
use chrono::Local;
use native_db::*;
use regex::Regex;
use uuid::Uuid;

pub struct  MessagingService<'a> {
    pub uuid: Uuid,
    pub discussion_name: String,
    pub tmp_messages: Vec<Message>,
    pub db: Database<'a>,
    pub driver: Box<dyn ErasedMessagingDriver>
}

impl MessagingService<'_> {
    pub fn load_tmp_messages(&mut self) -> anyhow::Result<()> {
        let r = self.db.r_transaction()?;
        let messages: Vec<Message> = r.scan().primary()?.all()?.try_collect()?;

        self.tmp_messages = messages;

        Ok(())
    }

    pub fn send_message(&mut self, text: String) -> anyhow::Result<()> {
        let text = text.trim();

        let regex = Regex::new("[\r\n]+")?;
        let text = regex.replace_all(text, "\n").to_string();

        let rw = self.db.rw_transaction()?;

        let message = Message {
            sender: None,
            text,
            timestamp: DateTime(Local::now()),
        };

        self.driver.send_message(&message)?;
        self.tmp_messages.push(message.clone());

        rw.insert(message)?;
        rw.commit()?;

        Ok(())
    }

    pub fn poll_received_messages(&mut self) -> anyhow::Result<bool> {
        let messages = self.driver.poll_received_messages()?;

        let has_received_messaged = !messages.is_empty();

        let rw = self.db.rw_transaction()?;
        for message in messages {
            rw.insert(message)?;
        }
        rw.commit()?;

        Ok(has_received_messaged)
    }
}