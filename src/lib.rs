use anyhow::Result;
use config::Config;
use std::error::Error;
use ui::core_ui::{generate_ui, shutdown, startup};
pub mod config;
mod ui {
    pub mod core_ui;
    pub mod keyboard_input;
    pub mod ui_helpers;
}
mod app;
mod conversion_utils;
mod enums {
    pub mod endian;
    pub mod format;
    pub mod inputmodes;
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
