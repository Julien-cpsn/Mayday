use crate::app::App;
use ratatui::prelude::{Constraint, Layout, Stylize};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

impl App<'_> {
    pub fn ui(&mut self, frame: &mut Frame) {
        let area = frame.area();

        let [app_name_area, main_area, app_state_area] = Layout::vertical(vec![
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1)
        ])
            .areas(area);


        let mayday = Paragraph::new("~ Mayday ~").centered().italic();
        frame.render_widget(mayday, app_name_area);

        self.render_main_layout(frame, main_area);
        self.render_app_state(frame, app_state_area);
    }
}