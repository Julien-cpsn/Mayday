use crate::models::service::MessagingService;
use log::{error, info, LevelFilter};
use ormlite::sqlite::SqliteConnection;

pub async fn start_active_worker(messaging_services: Vec<(MessagingService, SqliteConnection)>) -> anyhow::Result<()> {
    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .format_timestamp_secs()
        .init();


    let mut handles = vec![];

    for (mut messaging_service, mut db) in messaging_services {
        let handle = tokio::spawn(async move {
            info!("Starting messaging service {}", messaging_service.discussion_name);
            let err = messaging_service.active_poll_received_messages(&mut db).await;

            if let Err(err) = err {
                error!("{err}");
            }
        });

        handles.push(handle);
    }

    futures::future::join_all(handles).await;

    info!("Stopping worker...");

    Ok(())
}


pub async fn start_passive_worker(messaging_services: Vec<(MessagingService, SqliteConnection)>) -> anyhow::Result<()> {
    info!("Starting worker...");

    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .format_timestamp_secs()
        .init();


    let mut handles = vec![];

    for (mut messaging_service, mut db) in messaging_services {
        let handle = tokio::spawn(async move {
            info!("Starting messaging service {}", messaging_service.discussion_name);
            let err = messaging_service.passive_poll_received_messages(&mut db).await;

            if let Err(err) = err {
                error!("{err}");
            }
        });

        handles.push(handle);
    }

    futures::future::join_all(handles).await;

    info!("Stopping worker...");

    Ok(())
}
