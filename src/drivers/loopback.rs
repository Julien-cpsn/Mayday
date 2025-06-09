use crate::drivers::driver::MessagingDriver;
use crate::models::message::Message;
use chrono::Local;
use ratatui::prelude::Color;

const SENDER: &str = "Yourself";

pub struct LoopbackMessaging;

impl MessagingDriver for LoopbackMessaging {
    fn new() -> Self {
        LoopbackMessaging
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

    fn send_message(&mut self, messages: &mut Vec<Message>, text: String) -> anyhow::Result<()> {
        messages.push(Message {
            sender: None,
            text: text.clone(),
            timestamp: Local::now(),
        });

        messages.push(Message {
            sender: Some(String::from(SENDER)),
            text,
            timestamp: Local::now(),
        });

        Ok(())
    }

    fn poll_received_messages(&mut self) -> anyhow::Result<Vec<Message>> {
        Ok(vec![
            Message {
                sender: Some(String::from(SENDER)),
                text: String::from("T'es là mec ?"),
                timestamp: Local::now(),
            }
        ])
    }
}