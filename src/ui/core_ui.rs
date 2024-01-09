use crate::{
    app::App,
    config::Config,
    conversion_utils::{
        ascii::convert_bytes_to_ascii, from_eight_bytes::add_eight_bytes_as_number,
        from_four_bytes::add_bytes_as_number, from_one_byte_to_i8::add_byte_as_i8,
        from_three_bytes::add_three_bytes_as_number, from_two_bytes::add_two_bytes_as_number,
        hexadecimal::convert_bytes_to_hex,
    },
    enums::{endian::Endian, format::Format, inputmodes::InputMode},
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

use super::ui_helpers::{
    create_converted_values_list, create_help_message, create_input_paragraph,
    create_instructions_paragraph, create_list_of_formats, update, create_endianess_paragraph,
};

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
    let mut endianess = Endian::Big;
    let mut u32_numbers = Vec::new();
    let mut i32_numbers = Vec::new();
    let mut converted_binary_to_hex = Vec::new();
    let mut i8_numbers = Vec::new();
    let mut u16_numbers = Vec::new();
    let mut i16_numbers = Vec::new();
    let mut u24_numbers = Vec::new();
    let mut i24_numbers = Vec::new();
    let mut u64_numbers = Vec::new();
    let mut i64_numbers = Vec::new();
    let mut f32_numbers = Vec::new();
    let mut f64_numbers = Vec::new();
    let mut converted_binary_to_ascii = Vec::new();
    let format_list: Vec<Format> = Format::iter().collect();
    if config.little_endianess {
        endianess = Endian::Little;
    }
    add_bytes_as_number(&bytes_read, &mut u32_numbers, &endianess)?;
    add_bytes_as_number(&bytes_read, &mut i32_numbers, &endianess)?;
    add_two_bytes_as_number(&bytes_read, &mut u16_numbers, &endianess)?;
    add_two_bytes_as_number(&bytes_read, &mut i16_numbers, &endianess)?;
    add_three_bytes_as_number(&bytes_read, &mut u24_numbers, &endianess)?;
    add_three_bytes_as_number(&bytes_read, &mut i24_numbers, &endianess)?;
    add_eight_bytes_as_number(&bytes_read, &mut u64_numbers, &endianess)?;
    add_eight_bytes_as_number(&bytes_read, &mut i64_numbers, &endianess)?;
    add_bytes_as_number(&bytes_read, &mut f32_numbers, &endianess)?;
    add_eight_bytes_as_number(&bytes_read, &mut f64_numbers, &endianess)?;

    add_byte_as_i8(&bytes_read, &mut i8_numbers)?;
    convert_bytes_to_ascii(&bytes_read, &mut converted_binary_to_ascii)?;
    convert_bytes_to_hex(&bytes_read, &mut converted_binary_to_hex)?;
    let vec = &u32_numbers.clone();
    let mut app = App {
        bytes_read,
        should_quit: false,
        endianess, 
        converted_binary_to_u32: u32_numbers,
        converted_binary_to_i32: i32_numbers,
        converted_binary_to_hex,
        converted_binary_to_i8: i8_numbers,
        converted_binary_to_u16: u16_numbers,
        converted_binary_to_i16: i16_numbers,
        converted_binary_to_u24: u24_numbers,
        converted_binary_to_i24: i24_numbers,
        converted_binary_to_u64: u64_numbers,
        converted_binary_to_i64: i64_numbers,
        converted_binary_to_f32: f32_numbers,
        converted_binary_to_f64: f64_numbers,
        converted_binary_to_ascii,
        start_of_window: 0,
        end_of_window: 30,
        current_format: Format::Hex,
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
        Constraint::Percentage(25),
        Constraint::Percentage(5),
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
    let endianess_paragraph = create_endianess_paragraph(app);

    f.render_widget(current_format_paragraph, layout[0]);
    f.render_widget(endianess_paragraph,layout[1]);
    f.render_widget(list, layout[2]);
    f.render_widget(instructions_paragraph, layout[3]);
    f.render_widget(help_message, layout[5]);
    f.render_widget(input, layout[4]);

    if let InputMode::Editing = app.input_mode {
        f.set_cursor(
            layout[3].x + app.cursor_position as u16 + 1,
            layout[3].y + 1,
        )
    }
}
