use crate::app::App;
use crate::states::AppState;
use ratatui::prelude::{Line, Rect, Span, Stylize};
use ratatui::Frame;

impl App<'_> {
    pub fn render_app_state(&mut self, frame: &mut Frame, area: Rect) {
        let app_state = match self.state {
            AppState::Main => Line::from(vec![Span::raw("Main menu").on_dark_gray()]),
            AppState::MessagingServiceSelected => {
                let messaging_service = self.get_selected_messaging_services();
                let driver_name = messaging_service.driver.name();
                let discussion_name = &messaging_service.discussion_name;
                
                Line::from(vec![
                    Span::raw(driver_name).dark_gray(),
                    Span::raw(" > ").dark_gray(),
                    Span::raw(discussion_name).on_dark_gray()
                ])
            }
        };

        frame.render_widget(app_state, area);
    }
}