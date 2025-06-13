use crate::app::App;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::Frame;
use crate::states::AppState;

impl App<'_> {
    pub fn render_main_layout(&mut self, frame: &mut Frame, area: Rect) {
        let [messaging_services_area, discussion_area] = Layout::horizontal(vec![
            Constraint::Percentage(25),
            Constraint::Percentage(75),
        ])
            .areas(area);
        
        self.render_messaging_services_layout(frame, messaging_services_area);
        
        match self.state {
            AppState::Main => {}
            _ => self.render_discussion_layout(frame, discussion_area)
        }

        self.last_messages_area_size.0 = discussion_area.width.saturating_sub(4);
        self.last_messages_area_size.1 = discussion_area.height.saturating_sub(6);
    }
}