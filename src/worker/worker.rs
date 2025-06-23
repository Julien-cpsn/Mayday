use crate::models::service::MessagingService;
use log::LevelFilter;
use ormlite::sqlite::SqliteConnection;

pub async fn start_worker(mut messaging_services: Vec<(MessagingService, SqliteConnection)>) -> anyhow::Result<()> {
    println!("Starting worker...");

    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .format_timestamp_secs()
        .init();


    let handles = messaging_services
        .iter_mut()
        .map(|(messaging_service, db)| async {
            messaging_service.poll_received_messages(db).await
        });

    futures::future::join_all(handles).await;

    Ok(())
}
