use crate::models::config::DriverConfig;
use crate::models::message::Message;
use ratatui::prelude::Color;

pub trait MessagingDriver {
    type Config: DriverConfig + Sized;

    fn new(config: Self::Config) -> Self where Self: Sized;
    fn config(&self) -> &Self::Config;
    fn name(&self) -> &str;
    fn icon(&self) -> &str;
    fn color(&self) -> Color;
    fn send_message(&mut self, message: &Message) -> anyhow::Result<()>;
    fn poll_received_messages(&mut self) -> anyhow::Result<Vec<Message>>;
}

// Create a type-erased wrapper
pub trait ErasedMessagingDriver {
    fn config(&self) -> &dyn DriverConfig;
    fn name(&self) -> &str;
    fn icon(&self) -> &str;
    fn color(&self) -> Color;
    fn send_message(&mut self, message: &Message) -> anyhow::Result<()>;
    fn poll_received_messages(&mut self) -> anyhow::Result<Vec<Message>>;
}

impl<T: MessagingDriver> ErasedMessagingDriver for T {
    fn config(&self) -> &dyn DriverConfig {
        self.config()
    }

    fn name(&self) -> &str {
        MessagingDriver::name(self)
    }

    fn icon(&self) -> &str {
        MessagingDriver::icon(self)
    }

    fn color(&self) -> Color {
        MessagingDriver::color(self)
    }

    fn send_message(&mut self, message: &Message) -> anyhow::Result<()> {
        MessagingDriver::send_message(self, message)
    }

    fn poll_received_messages(&mut self) -> anyhow::Result<Vec<Message>> {
        MessagingDriver::poll_received_messages(self)
    }
}