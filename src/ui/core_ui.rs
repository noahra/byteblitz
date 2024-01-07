use crate::{
    config::Config,
    conversions::{ convert_bytes_to_ascii, add_bytes_as_number, Endian},
    app::App, enums::{format::Format, inputmodes::InputMode},
};
use anyhow::Result;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    layout::{Constraint, Layout},
    prelude::{CrosstermBackend, Frame, Terminal},
};
use std::error::Error;
use std::fs;
use strum::IntoEnumIterator;

use super::ui_helpers::{update, create_converted_values_list, create_list_of_formats, create_instructions_paragraph, create_help_message, create_input_paragraph};

pub fn startup() -> Result<()> {
    enable_raw_mode()?;
    execute!(std::io::stderr(), EnterAlternateScreen)?;
    Ok(())
}

pub fn shutdown() -> Result<()> {
    execute!(std::io::stderr(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
pub fn generate_ui(config: Config) -> Result<(), Box<dyn Error>> {
    let mut t = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    let bytes_read = fs::read(config.file_path)?;
    let mut u32_numbers = Vec::new();
    let mut i32_numbers = Vec::new();
    let mut converted_binary_to_ascii = Vec::new();
    let format_list: Vec<Format> = Format::iter().collect();

    add_bytes_as_number(&bytes_read, &mut u32_numbers, Endian::Big)?;
    add_bytes_as_number(&bytes_read, &mut i32_numbers, Endian::Big)?;
    convert_bytes_to_ascii(&bytes_read, &mut converted_binary_to_ascii)?;
    let vec = &u32_numbers.clone();
    let mut app = App {
        bytes_read,
        should_quit: false,
        converted_binary_to_u32: u32_numbers,
        converted_binary_to_i32: i32_numbers,
        converted_binary_to_ascii,
        start_of_window: 0,
        end_of_window: 30,
        current_format: Format::Uint32,
        format_list_index: 0,
        format_list,
        input: String::new(),
        input_mode: InputMode::Normal,
        cursor_position: 0,
        max_length: vec.len(),
    };

    loop {
        t.draw(|f| {
            ui(&mut app, f);
        })?;

        // application update
        update(&mut app)?;

        // application exit
        if app.should_quit {
            break;
        }
    }

    Ok(())
}

fn ui(app: &mut App, f: &mut Frame) {
    let constraints = [
        Constraint::Percentage(15),
        Constraint::Percentage(50),
        Constraint::Percentage(5),
        Constraint::Percentage(5),
        Constraint::Percentage(5),
        Constraint::Percentage(5),
    ];
    let layout = Layout::default()
        .constraints(constraints.as_ref())
        .split(f.size());

    let list = create_converted_values_list(app);
    let current_format_paragraph = create_list_of_formats(app);
    let instructions_paragraph = create_instructions_paragraph();
    let help_message = create_help_message(app);
    let input = create_input_paragraph(app);

    f.render_widget(current_format_paragraph, layout[0]);
    f.render_widget(list, layout[1]);
    f.render_widget(instructions_paragraph, layout[2]);
    f.render_widget(help_message, layout[4]);
    f.render_widget(input, layout[3]);

    if let InputMode::Editing = app.input_mode {
        f.set_cursor(
            layout[3].x + app.cursor_position as u16 + 1,
            layout[3].y + 1,
        )
    }
}
