use mayday::app::App;
use mayday::args::{Command, ARGS};
use mayday::files::config::parse_config_directory;
use mayday::models::stateful_messaging_services::StatefulMessagingServices;
use ratatui::widgets::ListState;
use mayday::worker::worker::{start_passive_worker};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let messaging_services = parse_config_directory().await?;
    
    match &ARGS.command {
        None => {
            let stateful_messaging_services = StatefulMessagingServices {
                messaging_services,
                list_state: ListState::default(),
            };
            
            let mut terminal = ratatui::init();
            let mut app = App::new(stateful_messaging_services).await?;

            app.run(&mut terminal).await?;

            ratatui::restore();
        }
        Some(command) => match command {
            Command::Worker => start_passive_worker(messaging_services).await?,
        }
    }

    Ok(())
}