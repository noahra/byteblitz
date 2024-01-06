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
fn ui(app: &mut App, f: &mut Frame) {
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

    let converted_values;

    match app.current_format {
        Format::Utf8 => {
            converted_values = create_display_list(&app.converted_binary_to_utf8.clone(), app);
        }
        Format::Uint32 => {
            converted_values = create_display_list(&app.converted_numbers.clone(), app);
        }
    }

    let list = List::new(converted_values)
        .block(
            Block::default()
                .title("Converted binary values")
                .borders(Borders::ALL),
        )
        .style(Style::default().fg(Color::Green))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true);

    let current_format_paragraph = Paragraph::new(Text::raw(format!(
        "Current Format: {:?}",
        app.current_format
    )))
    .style(Style::default().fg(Color::Yellow))
    .block(
        Block::default()
            .title("Current Format")
            .borders(Borders::ALL),
    );

    // Render the current format widget
    f.render_widget(current_format_paragraph, layout[0]);

    f.render_widget(list, layout[1]);

    let instructions_paragraph = Paragraph::new(Text::raw(
        "Use 'j' to move down, 'k' to move up in the list. Use 'h' and 'l' to switch between formats",
    ))
    .style(Style::default().fg(Color::Blue))
    .block(Block::default().title("Instructions").borders(Borders::ALL));

    f.render_widget(instructions_paragraph, layout[2]);

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
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, layout[4]);

    let input = Paragraph::new(app.input.as_str())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Line number"));
    f.render_widget(input, layout[3]);
    match app.input_mode {
        InputMode::Normal =>
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            {}

        InputMode::Editing => {
            // Make the cursor visible and ask ratatui to put it at the specified coordinates after
            // rendering
            f.set_cursor(
                // Draw the cursor at the current position in the input field.
                // This position is can be controlled via the left and right arrow key
                layout[3].x + app.cursor_position as u16 + 1,
                // Move one line down, from the border to the input line
                layout[3].y + 1,
            )
        }
    }
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
