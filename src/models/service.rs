use crate::drivers::driver::MessagingDriver;
use crate::models::message::Message;
use regex::Regex;

pub struct  MessagingService<'a> {
    pub discussion_name: &'a str,
    pub messages: Vec<Message>,
    pub driver: Box<dyn MessagingDriver>
}

impl MessagingService<'_> {
    pub fn send_message(&mut self, text: String) -> anyhow::Result<()> {
        let text = text.trim();

        let regex = Regex::new("[\r\n]+")?;
        let text = regex.replace_all(text, "\n").to_string();

        self.driver.send_message(&mut self.messages, text)
    }

    pub fn poll_received_messages(&mut self) -> anyhow::Result<bool> {
        let messages = self.driver.poll_received_messages()?;
        
        let has_received_messaged = !messages.is_empty();
        
        self.messages.extend(messages);
        
        Ok(has_received_messaged)
    }
}