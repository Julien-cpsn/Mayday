use crate::models::service::MessagingService;
use ratatui::widgets::ListState;

pub struct StatefulMessagingServices {
    pub messaging_services: Vec<MessagingService>,
    pub list_state: ListState
}