use chrono::Local;
use crate::app::App;
use crate::states::AppState;
use ratatui::layout::Margin;
use ratatui::prelude::{Alignment, Constraint, Layout, Line, Rect, Style, Stylize};
use ratatui::widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, Wrap};
use ratatui::Frame;
use textwrap::wrap;

impl App<'_> {
    pub fn render_discussion_layout(&mut self, frame: &mut Frame, area: Rect) {
        let [_, messages_area, text_area_area, _] = Layout::vertical(vec![
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(4),
            Constraint::Length(1)
        ])
            .areas(area);

        let inner_messages_area = messages_area.inner(Margin::new(2, 0));
        let inner_separator_area = text_area_area.inner(Margin::new(1, 0));
        let inner_text_area_area = text_area_area.inner(Margin::new(1, 1));
        self.last_messages_area_size.0 = inner_messages_area.width;
        self.last_messages_area_size.1 = inner_messages_area.height;

        let messaging_service = self.get_selected_messaging_services();

        let mut block = Block::bordered()
            .title_top(messaging_service.discussion_name.clone())
            .title_alignment(Alignment::Center);

        if self.state == AppState::Main {
            block = block.dark_gray();
        }

        frame.render_widget(block, area);

        let mut messages = vec![];
        let mut last_sender = &None;
        
        for message in &messaging_service.tmp_messages {
            let mut alignment = Alignment::Right;

            let max_length = self.get_max_line_length(&message.text);
            let lines = wrap(&message.text, max_length);

            if let Some(sender) = &message.sender {
                alignment = Alignment::Left;

                if last_sender != &message.sender {
                    messages.push(
                        Line::raw(sender)
                            .bold()
                            .fg(messaging_service.driver.color())
                            .alignment(alignment)
                    );
                }

                for line in lines {
                    messages.push(
                        Line::raw(format!("{:length$}", line, length = max_length))
                            .white()
                            .bg(messaging_service.driver.color())
                            .alignment(alignment)
                    );
                }
            }
            else {
                for line in lines {
                    messages.push(
                        Line::raw(format!("{:length$}", line, length = max_length))
                            .white()
                            .on_gray()
                            .alignment(alignment)
                    );
                }
            }

            let timestamp_format = match Local::now().date_naive() == message.timestamp.0.date_naive() {
                true => "%H:%M",
                false => "%H:%M %d/%m/%Y"
            };
            
            messages.push(
                Line::raw(message.timestamp.0.format(timestamp_format).to_string())
                    .dark_gray()
                    .dim()
                    .alignment(alignment)
            );
            
            last_sender = &message.sender;
        }

        let messages_paragraph = Paragraph::new(messages)
            .scroll((
                self.discussion_scrollbar.scroll,
                0
            ))
            .wrap(Wrap { trim: false });
        frame.render_widget(&messages_paragraph, inner_messages_area);

        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .style(Style::new().dark_gray());
        frame.render_stateful_widget(scrollbar, messages_area, &mut self.discussion_scrollbar.state);

        let separator_block = Block::new().borders(Borders::TOP).dark_gray();
        frame.render_widget(separator_block, inner_separator_area);

        frame.render_widget(&self.message_input, inner_text_area_area);
    }
}