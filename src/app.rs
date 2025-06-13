use crate::models::stateful_messaging_services::StatefulMessagingServices;
use crate::models::stateful_scrollbar::StatefulScrollbar;
use crate::states::AppState;
use ratatui::DefaultTerminal;
use tui_textarea::TextArea;

pub struct App<'a> {
    pub state: AppState,
    pub should_quit: bool,

    pub stateful_messaging_services: StatefulMessagingServices,

    pub discussion_scrollbar: StatefulScrollbar,
    pub last_messages_area_size: (u16, u16),
    pub should_update_discussion_scrollbar: bool,
    pub message_input: TextArea<'a>,
}

impl<'a> App<'a> {
    pub async fn new(stateful_messaging_services: StatefulMessagingServices) -> anyhow::Result<Self> {
        let mut app = App {
            state: AppState::Main,
            should_quit: false,
            stateful_messaging_services,

            discussion_scrollbar: StatefulScrollbar::default(),
            last_messages_area_size: (0, 0),
            should_update_discussion_scrollbar: false,
            message_input: TextArea::new(vec![]),
        };

        app.reset_message_input();
        app.load_all_messages().await?;

        Ok(app)
    }
    
    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> anyhow::Result<()> {
        loop {
            terminal.draw(|frame| self.ui(frame))?;
            self.handle_events().await?;
            
            if self.should_quit {
                return Ok(());
            }
        }
    }
}