use crate::models::service::MessagingService;
use std::time::Duration;
use log::{info, LevelFilter};
use tokio::time::sleep;

pub const TICK_RATE: Duration = Duration::from_millis(10000);

pub async fn start_worker(mut messaging_services: Vec<MessagingService>) -> anyhow::Result<()> {
    println!("Starting worker...");

    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .format_timestamp_secs()
        .init();


    loop {
        for messaging_service in messaging_services.iter_mut() {
            info!("Polling received messages for \"{}\"", messaging_service.discussion_name);
            messaging_service.poll_received_messages().await?;
        }
        sleep(TICK_RATE).await;
    }
}