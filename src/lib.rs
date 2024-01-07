use anyhow::Result;
use config::Config;
use ui::core_ui::{startup, generate_ui, shutdown};
use std::error::Error;
pub mod config;
mod conversions;

mod app;
mod ui {
    pub mod ui_helpers;
    pub mod core_ui;
    pub mod keyboard_input;
}
mod enums {
    pub mod inputmodes;
    pub mod format;
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;

    startup()?;
    let status = generate_ui(config);
    shutdown()?;
    status?;

    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}
