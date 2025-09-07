use crate::app::App;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::Frame;
use ratatui::prelude::Stylize;
use ratatui::widgets::Paragraph;
use crate::states::AppState;

const LOGO: &str = " ▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄
████                         ████
██ ███                     ███ ██
██   ███                 ███   ██
██     ██               ███    ██
██      ███           ███      ██
██        █████   █████        ██
██      ███   █████   ███      ██
██    ███               ████   ██
██ ████                   ████ ██
████                         ████
  ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀  ";

impl App<'_> {
    pub fn render_main_layout(&mut self, frame: &mut Frame, area: Rect) {
        let [messaging_services_area, discussion_area] = Layout::horizontal(vec![
            Constraint::Percentage(25),
            Constraint::Percentage(75),
        ])
            .areas(area);
        
        self.render_messaging_services_layout(frame, messaging_services_area);
        
        match self.state {
            AppState::Main => self.render_homepage(frame, discussion_area),
            _ => self.render_discussion_layout(frame, discussion_area)
        }

        self.last_messages_area_size.0 = discussion_area.width.saturating_sub(4);
        self.last_messages_area_size.1 = discussion_area.height.saturating_sub(6);
    }

    fn render_homepage(&mut self, frame: &mut Frame, area: Rect) {
        let height_percentage = 12 * 100 / area.height;

        let title_layout = Layout::vertical([
            Constraint::Percentage(50 - (height_percentage / 2)),
            Constraint::Length(12),
            Constraint::Percentage(50 - (height_percentage / 2)),
        ])
            .split(area);

        let logo_par = Paragraph::new(LOGO).dark_gray().centered();
        frame.render_widget(logo_par, title_layout[1]);
    }
}