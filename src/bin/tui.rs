use ratatui::widgets::ListState;
use mayday::app::App;
use mayday::files::config::parse_config_directory;
use mayday::models::stateful_messaging_services::StatefulMessagingServices;

fn main() -> anyhow::Result<()> {
    let messaging_service = StatefulMessagingServices {
        messaging_services: parse_config_directory(),
        list_state: ListState::default(),
    };

    let mut terminal = ratatui::init();
    let mut app = App::new(messaging_service)?;

    app.run(&mut terminal)?;

    ratatui::restore();

    Ok(())
}