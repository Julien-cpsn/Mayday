use crate::models::driver_config::DriverConfig;
use crate::models::message::Message;
use async_trait::async_trait;
use ormlite::sqlite::SqliteConnection;
use ratatui::prelude::Color;

#[async_trait]
pub trait MessagingDriver : Send + Sync {
    type Config: DriverConfig + Sized + Clone;

    async fn new(config: Self::Config) -> anyhow::Result<Self> where Self: Sized;
    fn config(&self) -> &Self::Config;
    fn name(&self) -> &str;
    fn icon(&self) -> &str;
    fn color(&self) -> Color;
    async fn send_message(&mut self, db: &mut SqliteConnection, message: &Message) -> anyhow::Result<()>;
    async fn active_poll_received_messages(&mut self, db: &mut SqliteConnection) -> anyhow::Result<()>;
    async fn passive_poll_received_messages(&mut self, db: &mut SqliteConnection) -> anyhow::Result<()>;
}

#[async_trait]
pub trait ErasedMessagingDriver : Send + Sync {
    fn config(&self) -> &dyn DriverConfig;
    fn name(&self) -> &str;
    fn icon(&self) -> &str;
    fn color(&self) -> Color;
    async fn send_message(&mut self, db: &mut SqliteConnection, message: &Message) -> anyhow::Result<()>;
    async fn active_poll_received_messages(&mut self, db: &mut SqliteConnection) -> anyhow::Result<()>;
    async fn passive_poll_received_messages(&mut self, db: &mut SqliteConnection) -> anyhow::Result<()>;
}

#[async_trait]
impl<T: MessagingDriver + Send + Sync> ErasedMessagingDriver for T {
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

    async fn send_message(&mut self, db: &mut SqliteConnection, message: &Message) -> anyhow::Result<()> {
        MessagingDriver::send_message(self, db, message).await
    }

    async fn active_poll_received_messages(&mut self, db: &mut SqliteConnection) -> anyhow::Result<()> {
        MessagingDriver::active_poll_received_messages(self, db).await
    }
    
    async fn passive_poll_received_messages(&mut self, db: &mut SqliteConnection) -> anyhow::Result<()> {
        MessagingDriver::passive_poll_received_messages(self, db).await
    }
}