use crossterm::event::{self};
use ratatui::{widgets::{List, Block, Borders, ListItem, ListDirection, Paragraph}, style::{Modifier, Color, Style, Stylize}, text::{Text, Line}};
use crate::{ app::App, enums::{inputmodes::InputMode, format::Format}};

use super::keyboard_input::{handle_normal_mode_keys, handle_editing_mode_keys};

pub fn update(app: &mut App) -> Result<(), std::io::Error> {
    if event::poll(std::time::Duration::from_millis(250))? {
        if let event::Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match app.input_mode {
                    InputMode::Normal => handle_normal_mode_keys(app, key.code)?,
                    InputMode::Editing => handle_editing_mode_keys(app, key.code)?,
                }
            }
        }
    }
    Ok(())
}


pub fn create_display_list<T: std::fmt::Display>(
    vector_to_be_converted: &[T],
    app: &mut App,
) -> Vec<String> {
    let max_index = app.start_of_window + app.end_of_window;
    let max_index_width = max_index.to_string().len();
    app.max_length = vector_to_be_converted.len();

    vector_to_be_converted
        .iter()
        .enumerate() // Get the index and value
        .skip(app.start_of_window) // Skip to the starting window index.
        .take(app.end_of_window - app.start_of_window) // Take the range from start to end of the window.
        .map(|(index, n)| format!("{:width$}. {}", index + 1, n, width = max_index_width))
        .collect()
}


pub fn create_converted_values_list(app: &mut App) -> List<'static> {
    let converted_values = match app.current_format {
        Format::Ascii => create_display_list(&app.converted_binary_to_ascii.clone(), app),
        Format::Uint32 => create_display_list(&app.converted_binary_to_u32.clone(), app),
        Format::Int32 => create_display_list(&app.converted_binary_to_i32.clone(), app),
        Format::Int8 => create_display_list(&app.converted_binary_to_i8.clone(), app),
        Format::Uint8 => create_display_list(&app.bytes_read.clone(), app),
        Format::Int16 => create_display_list(&app.converted_binary_to_i16.clone(), app),
        Format::Uint16 => create_display_list(&app.converted_binary_to_u16.clone(), app),
        Format::Int24 => create_display_list(&app.converted_binary_to_u24.clone(), app),
        Format::Uint24 => create_display_list(&app.converted_binary_to_i24.clone(), app),
        Format::Uint64 => create_display_list(&app.converted_binary_to_u64.clone(), app),
        Format::Int64 => create_display_list(&app.converted_binary_to_i64.clone(), app),
        Format::F32 => create_display_list(&app.converted_binary_to_f32.clone(), app),
        Format::F64 => create_display_list(&app.converted_binary_to_f64.clone(), app),
        Format::Hex => create_display_list(&app.converted_binary_to_hex.clone(), app),
    };

    List::new(converted_values)
        .block(
            Block::default()
                .title(format!(
                    "Converted binary values - Total: {}",
                    app.max_length
                ))
                .borders(Borders::ALL),
        )
        .style(Style::default().fg(Color::Green))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true)
}

pub fn create_list_of_formats(app: &App) -> List<'static> {
    let mut vector_of_formats: Vec<ListItem> = Vec::new();

    for (index, element) in app.format_list.iter().enumerate() {
        if index == app.format_list_index {
            let format_paragraph = ListItem::new(Text::raw(format!("{:?}", element)))
                .style(Style::default().fg(Color::Yellow));

            vector_of_formats.push(format_paragraph);
        } else {
            let format_paragraph = ListItem::new(Text::raw(format!("{:?}", element)))
                .style(Style::default().fg(Color::White));

            vector_of_formats.push(format_paragraph);
        }
    }
    let list = List::new(vector_of_formats)
        .block(Block::default().title("Current format").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom);

    list
}

pub fn create_endianess_paragraph(app: &App) -> Paragraph<'_> {
    Paragraph::new(Text::raw(format!("{:?}", app.endianess)))
        .style(Style::default().fg(Color::Green))
        .block(Block::default().title("Endianess").borders(Borders::ALL))
}
pub fn create_instructions_paragraph() -> Paragraph<'static> {
    Paragraph::new(Text::raw(
        "Use 'j' to move down, 'k' to move up in the list. Use 'h' and 'l' to switch between formats",
    ))
    .style(Style::default().fg(Color::Blue))
    .block(Block::default().title("Instructions").borders(Borders::ALL))
}
pub fn create_help_message(app: &App) -> Paragraph<'static> {
    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                "Press ".into(),
                "q".bold(),
                " to exit, ".into(),
                "e".bold(),
                " to type in a line number to navigate to.".into(),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => (
            vec![
                "Press ".into(),
                "Esc".bold(),
                " to turn off search field, ".into(),
                "Enter".bold(),
                " to go to the line number".into(),
            ],
            Style::default(),
        ),
    };
    let mut text = Text::from(Line::from(msg));
    text.patch_style(style);
    Paragraph::new(text)
}

// Function to create the input paragraph
pub fn create_input_paragraph(app: &App) -> Paragraph {
    Paragraph::new(app.input.as_str())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Line number"))
}
