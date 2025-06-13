use crate::app::App;
use crate::models::service::MessagingService;

#[derive(PartialEq)]
pub enum AppState {
    Main,
    MessagingServiceSelected,
}

impl<'a> App<'a> {
    pub fn to_main_state(&mut self) {
        self.state = AppState::Main;
    }
    
    pub fn to_messaging_service_selected_state(&mut self) {
        if self.stateful_messaging_services.list_state.selected().is_none() {
            return;
        }

        self.should_update_discussion_scrollbar = true;

        self.state = AppState::MessagingServiceSelected;
    }
    
    pub fn get_selected_messaging_services(&self) -> &MessagingService {
        &self.stateful_messaging_services.messaging_services[self.stateful_messaging_services.list_state.selected().unwrap()]
    }

    pub fn get_selected_messaging_services_mut(&mut self) -> &mut MessagingService {
        self.stateful_messaging_services.messaging_services.get_mut(self.stateful_messaging_services.list_state.selected().unwrap()).unwrap()
    }
}