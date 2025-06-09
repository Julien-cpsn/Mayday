use ratatui::widgets::ListState;
use crate::models::service::MessagingService;

pub struct StatefulMessagingServices<'a> {
    pub messaging_services: Vec<MessagingService<'a>>,
    pub list_state: ListState
}