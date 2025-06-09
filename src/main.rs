mod ui;
mod drivers;
mod app;
mod models;
mod states;
mod events;
mod business_logic;

use crate::app::App;

fn main() -> anyhow::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::new();

    app.run(&mut terminal)?;

    ratatui::restore();

    Ok(())
}