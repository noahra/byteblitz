use crate::{
    config::Config,
    conversions::{add_bytes_as_u32, convert_bytes_to_utf8},
    format::Format,
    inputmodes::InputMode, ui_helpers::{update, App},
};
use anyhow::Result;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    layout::{Constraint, Layout},
    prelude::{CrosstermBackend, Frame, Terminal},
    style::{Color, Modifier, Style, Stylize as _},
    text::{Line, Text},
    widgets::{Block, Borders, List, Paragraph},
};
use std::error::Error;
use std::fs;
use strum::IntoEnumIterator;


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




pub fn generate_ui(config: Config) -> Result<(), Box<dyn Error>> {
    let mut t = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    let bytes_read = fs::read(config.file_path)?;
    let mut u32_numbers = Vec::new();
    let mut converted_binary_to_utf8 = Vec::new();
    let format_list: Vec<Format> = Format::iter().collect();

    add_bytes_as_u32(&bytes_read, &mut u32_numbers)?;
    convert_bytes_to_utf8(&bytes_read, &mut converted_binary_to_utf8)?;
    let vec = &u32_numbers.clone();
    let mut app = App {
        bytes_read,
        should_quit: false,
        converted_numbers: u32_numbers,
        converted_binary_to_utf8,
        start_of_window: 0,
        end_of_window: 36,
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








// Function to create the list of converted values
fn create_converted_values_list(app: &mut App) -> List<'static> {
    let converted_values = match app.current_format {
        Format::Utf8 => create_display_list(&app.converted_binary_to_utf8.clone(), app),
        Format::Uint32 => create_display_list(&app.converted_numbers.clone(), app),
    };

    List::new(converted_values)
        .block(
            Block::default()
                .title("Converted binary values")
                .borders(Borders::ALL),
        )
        .style(Style::default().fg(Color::Green))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true)
}

// Function to create the current format paragraph
fn create_current_format_paragraph(app: &App) -> Paragraph<'static> {
    Paragraph::new(Text::raw(format!(
        "Current Format: {:?}",
        app.current_format
    )))
    .style(Style::default().fg(Color::Yellow))
    .block(
        Block::default()
            .title("Current Format")
            .borders(Borders::ALL),
    )
}

// Function to create the instructions paragraph
fn create_instructions_paragraph() -> Paragraph<'static> {
    Paragraph::new(Text::raw(
        "Use 'j' to move down, 'k' to move up in the list. Use 'h' and 'l' to switch between formats",
    ))
    .style(Style::default().fg(Color::Blue))
    .block(Block::default().title("Instructions").borders(Borders::ALL))
}

// Function to create the help message
fn create_help_message(app: &App) -> Paragraph<'static> {
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
                "to go to the line number".into(),
            ],
            Style::default(),
        ),
    };
    let mut text = Text::from(Line::from(msg));
    text.patch_style(style);
    Paragraph::new(text)
}

// Function to create the input paragraph
fn create_input_paragraph(app: &mut App) -> Paragraph<> {
    Paragraph::new(app.input.as_str())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Line number"))
}

// Refactored ui function
fn ui(app: &mut App, f: &mut Frame) {
    // Define constraints and layout
    let constraints = [
        Constraint::Percentage(5),
        Constraint::Percentage(50),
        Constraint::Percentage(5),
        Constraint::Percentage(5),
        Constraint::Percentage(5),
        Constraint::Percentage(5),
        Constraint::Percentage(5),
    ];
    let layout = Layout::default()
        .constraints(constraints.as_ref())
        .split(f.size());

    // Create widgets
    let list = create_converted_values_list(app);
    let current_format_paragraph = create_current_format_paragraph(app);
    let instructions_paragraph = create_instructions_paragraph();
    let help_message = create_help_message(app);
    let input = create_input_paragraph(app);

    // Render widgets
    f.render_widget(current_format_paragraph, layout[0]);
    f.render_widget(list, layout[1]);
    f.render_widget(instructions_paragraph, layout[2]);
    f.render_widget(help_message, layout[4]);
    f.render_widget(input, layout[3]);

    // Handle cursor for input mode
    if let InputMode::Editing = app.input_mode {
        f.set_cursor(layout[3].x + app.cursor_position as u16 + 1, layout[3].y + 1)
    }
}
