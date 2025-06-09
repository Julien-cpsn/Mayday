use crate::drivers::driver::MessagingDriver;
use crate::drivers::loopback::LoopbackMessaging;
use crate::models::message::Message;
use crate::models::service::MessagingService;
use crate::models::stateful_messaging_services::StatefulMessagingServices;
use crate::models::stateful_scrollbar::StatefulScrollbar;
use crate::states::AppState;
use chrono::DateTime;
use ratatui::widgets::ListState;
use ratatui::DefaultTerminal;
use tui_textarea::TextArea;

pub struct App<'a> {
    pub state: AppState,
    pub should_quit: bool,

    pub stateful_messaging_services: StatefulMessagingServices<'a>,

    pub discussion_scrollbar: StatefulScrollbar,
    pub last_messages_area_size: (u16, u16),
    pub message_input: TextArea<'a>,
}

impl App<'_> {
    pub fn new() -> Self {
        let mut app = App {
            state: AppState::Main,
            should_quit: false,
            stateful_messaging_services: StatefulMessagingServices {
                messaging_services: vec![
                    MessagingService {
                        discussion_name: "Yourself",
                        messages: vec![
                            Message {
                                sender: Some("Yourself".to_string()),
                                text: "Yooo mec ça fait longtemgue".to_string(),
                                timestamp: DateTime::default(),
                            },
                            Message {
                                sender: None,
                                text: "Oui c moi".to_string(),
                                timestamp: DateTime::default(),
                            },
                        ],
                        driver: Box::new(LoopbackMessaging::new()),
                    }
                ],
                list_state: ListState::default(),
            },

            discussion_scrollbar: StatefulScrollbar::default(),
            last_messages_area_size: (0, 0),
            message_input: TextArea::new(vec![]),
        };

        app.reset_message_input();
        
        app
    }
    
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> anyhow::Result<()> {
        loop {
            terminal.draw(|frame| self.ui(frame))?;
            self.handle_events()?;
            
            if self.should_quit {
                return Ok(());
            }
        }
    }
}