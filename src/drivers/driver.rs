use crate::models::message::Message;
use ratatui::prelude::Color;

pub trait MessagingDriver {
    fn new() -> Self where Self: Sized;
    fn name(&self) -> &str;
    fn icon(&self) -> &str;
    fn color(&self) -> Color;
    fn send_message(&mut self, messages: &mut Vec<Message>, message: String) -> anyhow::Result<()>;
    fn poll_received_messages(&mut self) -> anyhow::Result<Vec<Message>>;
}