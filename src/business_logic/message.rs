use crate::app::App;
use ratatui::prelude::{Style, Stylize};
use rayon::prelude::*;
use textwrap::wrap;
use tui_textarea::TextArea;

impl App<'_> {
    pub fn reset_message_input(&mut self) {
        self.message_input = TextArea::new(vec![]);
        self.message_input.set_style(Style::new().white());
        self.message_input.set_placeholder_text("Type...");
    }

    pub fn load_all_messages(&mut self) -> anyhow::Result<()> {
        for messaging_service in self.stateful_messaging_services.messaging_services.iter_mut() {
            messaging_service.load_tmp_messages()?;
        }

        Ok(())
    }

    pub fn send_message(&mut self) {
        let text = &self.message_input.lines().join("\n");

        if text.trim().is_empty() {
            return;
        }

        {
            let messaging_service = self.get_selected_messaging_services_mut();
            messaging_service.send_message(text.clone()).unwrap();
        }

        self.reset_message_input();
        self.update_discussion_scrollbar();
    }

    pub fn get_messages_lines_count(&self) -> usize {
        let mut line_count = 0;
        let messaging_service = self.get_selected_messaging_services();

        let mut last_sender = &None;

        for message in &messaging_service.tmp_messages {
            let max_length = self.get_max_line_length(&message.text);
            let lines = wrap(&message.text, max_length);

            match message.sender {
                None => line_count += lines.len() + 1,
                Some(_) => match last_sender == &message.sender {
                    true => line_count += lines.len() + 1,
                    false => line_count += lines.len() + 2,
                }
            }

            last_sender = &message.sender;
        }

        line_count
    }

    pub fn get_max_line_length(&self, text: &str) -> usize {
        let mut max_length = text.par_lines().fold_with(0, |acc, line| acc + line.chars().count()).sum();

        if self.last_messages_area_size.0 > 0 {
            let max_width = (0.75 * self.last_messages_area_size.0 as f32) as usize + 1;
            if max_length > max_width {
                max_length = max_width;
            }
        }

        max_length
    }
}