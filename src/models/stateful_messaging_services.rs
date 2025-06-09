use crate::models::service::MessagingService;
use ratatui::widgets::ListState;

pub struct StatefulMessagingServices<'a> {
    pub messaging_services: Vec<MessagingService<'a>>,
    pub list_state: ListState
}