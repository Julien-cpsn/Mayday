use crate::app::App;
use crate::models::service::MessagingService;
use crate::states::AppState;
use ratatui::layout::{Alignment, Rect};
use ratatui::prelude::{Line, Span, Style, Stylize};
use ratatui::widgets::{Block, List};
use ratatui::Frame;

impl App<'_> {
    pub fn render_messaging_services_layout(&mut self, frame: &mut Frame, area: Rect) {
        let messaging_services_items: Vec<Line> = self.stateful_messaging_services
            .messaging_services
            .iter()
            .map(|messaging_service: &MessagingService| Line::from(vec![
                Span::raw(messaging_service.driver.icon()).white().bg(messaging_service.driver.color()),
                format!(" {}", messaging_service.discussion_name).into()
            ]))
            .collect();

        let mut messaging_services_block = Block::bordered()
            .title_top("Messaging Services")
            .title_alignment(Alignment::Left);
        
        if self.state != AppState::Main {
            messaging_services_block = messaging_services_block.dark_gray();
        }
        
        let messaging_services_list = List::new(messaging_services_items)
            .highlight_symbol(">")
            .highlight_style(Style::new().bold())
            .block(messaging_services_block);

        frame.render_stateful_widget(
            messaging_services_list,
            area,
            &mut self.stateful_messaging_services.list_state
        );
    }
}