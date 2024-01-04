use anyhow::Result;
use crossterm::{
    event::{self, Event::Key, KeyCode::Char},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    layout::{Constraint, Layout},
    prelude::{CrosstermBackend, Frame, Terminal},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, Paragraph}, text::Text,
};
use std::convert::From;
use std::error::Error;
use std::fs;

pub struct Config {
    pub file_path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let file_path = args[1].clone();

        Ok(Config { file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;

    startup()?;
    let status = generate_ui(config);
    shutdown()?;
    status?;

    // shutdown down: reset terminal back to original state
    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}

pub fn convert_to_u32(bytes: [u8; 4]) -> Option<u32> {
    if bytes.len() >= 4 {
        let b = [bytes[0], bytes[1], bytes[2], bytes[3]];
        Some(u32::from_be_bytes(b))
    } else {
        None
    }
}
fn startup() -> Result<()> {
    enable_raw_mode()?;
    execute!(std::io::stderr(), EnterAlternateScreen)?;
    Ok(())
}

fn shutdown() -> Result<()> {
    execute!(std::io::stderr(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn ui(app: &App, f: &mut Frame) {
    let constraints = [
        Constraint::Percentage(25),
        Constraint::Percentage(5),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
    ];

    // Create a layout
    let layout = Layout::default()
        .constraints(constraints.as_ref())
        .split(f.size());

        let max_index = app.start_of_window + app.end_of_window;
        let max_index_width = max_index.to_string().len();
    
        // Convert the u32 numbers to numbered strings and collect them into a Vec
        let number_strings: Vec<String> = app
            .converted_numbers
            .iter()
            .enumerate() // Get the index and value
            .skip(app.start_of_window) // Skip to the starting window index.
            .take(app.end_of_window - app.start_of_window) // Take the range from start to end of the window.
            .map(|(index, n)| {
                // Format with padded index number.
                format!("{:width$}. {}", index + 1, n, width = max_index_width)
            })
            .collect();

    let list = List::new(number_strings)
        .block(
            Block::default()
                .title("Converted binary values")
                .borders(Borders::ALL),
        )
        .style(Style::default().fg(Color::Green))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true);

    // Render the table in the layout
    f.render_widget(list, layout[0]);

  
    // Create a paragraph for the instructions
    let instructions_paragraph = Paragraph::new(Text::raw("Use 'j' to move up, 'k' to move down in the list."))
        .style(Style::default().fg(Color::Blue))
        .block(
            Block::default()
                .title("Instructions")
                .borders(Borders::ALL),
        );

    // Render the instructions paragraph
    // Assuming it should be in the second part of the layout
    f.render_widget(instructions_paragraph, layout[1]);

}

fn update(app: &mut App) -> Result<()> {
    if event::poll(std::time::Duration::from_millis(250))? {
        if let Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    Char('q') => app.should_quit = true,
                    Char('k') => {
                        app.start_of_window += 1;
                        app.end_of_window += 1;
                    }
                    Char('j') if app.start_of_window > 0 => {
                        app.start_of_window -= 1;
                        app.end_of_window -= 1;
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}
struct App {
    should_quit: bool,
    converted_numbers: Vec<u32>,
    start_of_window: usize,
    end_of_window: usize,
}

fn generate_ui(config: Config) -> Result<(), Box<dyn Error>> {
    // ratatui terminal
    let mut t = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    let bytes_read = fs::read(config.file_path)?;
    let mut u32_numbers = Vec::new();

    add_bytes_as_u32(&bytes_read, &mut u32_numbers)?;
    let mut app = App {
        should_quit: false,
        converted_numbers: u32_numbers,
        start_of_window: 0,
        end_of_window: 14,
    };

    loop {
        t.draw(|f| {
            ui(&app, f);
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

fn add_bytes_as_u32(
    bytes: &[u8],
    u32_numbers: &mut Vec<u32>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Calculate the maximum index that is a multiple of 4 and within the byte array's bounds
    let max_index = bytes.len() - (bytes.len() % 4);

    // Iterate over the bytes in steps of 4 up to the max_index
    for i in (0..max_index).step_by(4) {
        let chunk = [bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]];
        if let Some(u32_integer) = convert_to_u32(chunk) {
            u32_numbers.push(u32_integer);
        } else {
            return Err(From::from("Failed to convert bytes to u32."));
        }
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u32_conversion() {
        let result = convert_to_u32([0x89, 0x50, 0x4E, 0x47]);
        assert_eq!(result, Some(2303741511));
    }
}
