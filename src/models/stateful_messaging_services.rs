use ormlite::sqlite::SqliteConnection;
use crate::models::service::MessagingService;
use ratatui::widgets::ListState;

pub struct StatefulMessagingServices {
    pub messaging_services: Vec<(MessagingService, SqliteConnection)>,
    pub list_state: ListState
}