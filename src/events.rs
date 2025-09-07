use std::time::Duration;
use crokey::{crossterm, key, KeyCombination};
use crokey::crossterm::event;
use crokey::crossterm::event::{Event, KeyCode, KeyEvent};
use crokey::OneToThree::One;
use tui_textarea::CursorMove;
use crate::app::App;
use crate::states::AppState;

const TICK_RATE: Duration = Duration::from_millis(1000);

impl App<'_> {
    pub async fn handle_events(&mut self) -> anyhow::Result<()> {
        if event::poll(TICK_RATE)? {
            if let Ok(event) = crossterm::event::read() {
                if let Event::Key(key) = event {
                    self.handle_event(key).await?;
                }
            }
        }
        else {
            for (index, (messaging_service, db)) in self.stateful_messaging_services.messaging_services.iter_mut().enumerate() {
                messaging_service.active_poll_received_messages(db).await?;
                if messaging_service.try_load_messages(db).await? && self.stateful_messaging_services.list_state.selected().is_some() && self.stateful_messaging_services.list_state.selected().unwrap() == index {
                    self.should_update_discussion_scrollbar = true;
                }
            }
        }

        if self.should_update_discussion_scrollbar {
            self.update_discussion_scrollbar();
            self.should_update_discussion_scrollbar = false;
        }

        Ok(())
    }

    async fn handle_event(&mut self, key: KeyEvent) -> anyhow::Result<()> {
        let key_combination: KeyCombination = KeyCombination::from(key);

        //dbg!(&key_combination);

        match key_combination {
            key!(ctrl-c) => self.should_quit = true,
            _ => match self.state {
                AppState::Main => match key_combination {
                    key!(down) => self.stateful_messaging_services.list_state.select_next(),
                    key!(up) => self.stateful_messaging_services.list_state.select_previous(),
                    key!(enter) => self.to_messaging_service_selected_state(),
                    _ => {}
                },
                AppState::MessagingServiceSelected => match key_combination {
                    key!(esc) => self.to_main_state(),
                    key!(alt-enter) | key!(ctrl-enter) => self.send_message().await,
                    key!(delete) => { self.message_input.delete_next_char(); },
                    key!(backspace) => { self.message_input.delete_char(); },
                    key!(ctrl-shift-C) => self.message_input.copy(),
                    key!(ctrl-shift-V) => { self.message_input.paste(); }
                    key!(ctrl-shift-X) => { self.message_input.cut(); }
                    key!(ctrl-z) => { self.message_input.undo(); }
                    key!(ctrl-shift-Z) => { self.message_input.redo(); }
                    key!(up) => self.message_input.move_cursor(CursorMove::Up),
                    key!(down) => self.message_input.move_cursor(CursorMove::Down),
                    key!(left) => self.message_input.move_cursor(CursorMove::Back),
                    key!(right) => self.message_input.move_cursor(CursorMove::Forward),
                    key!(ctrl-up) => self.message_input.move_cursor(CursorMove::Top),
                    key!(ctrl-down) => self.message_input.move_cursor(CursorMove::Bottom),
                    key!(ctrl-left) => self.message_input.move_cursor(CursorMove::WordBack),
                    key!(ctrl-right) => self.message_input.move_cursor(CursorMove::WordForward),
                    key!(home) => self.message_input.move_cursor(CursorMove::Head),
                    key!(end) => self.message_input.move_cursor(CursorMove::End),
                    key!(tab) => { self.message_input.insert_tab(); },
                    key!(enter) => self.message_input.insert_newline(),
                    key!(pagedown) | key!(shift-pagedown) => self.discussion_scrollbar.page_down(),
                    key!(pageup) | key!(shift-pageup) => self.discussion_scrollbar.page_up(),
                    KeyCombination { codes: One(KeyCode::Char(char)), .. } => self.message_input.insert_char(char),
                    _ => {}
                }
            },
        }

        Ok(())
    }
}