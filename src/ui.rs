use crate::{config::Config, conversions::add_bytes_as_u32};
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
    text::Text,
    widgets::{Block, Borders, List, Paragraph},
};
use std::error::Error;
use std::fs;

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

fn ui(app: &App, f: &mut Frame) {
    let constraints = [
        Constraint::Percentage(60),
        Constraint::Percentage(5),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
    ];

    let layout = Layout::default()
        .constraints(constraints.as_ref())
        .split(f.size());

    let max_index = app.start_of_window + app.end_of_window;
    let max_index_width = max_index.to_string().len();

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

    f.render_widget(list, layout[0]);

    let instructions_paragraph = Paragraph::new(Text::raw(
        "Use 'j' to move down, 'k' to move up in the list.",
    ))
    .style(Style::default().fg(Color::Blue))
    .block(Block::default().title("Instructions").borders(Borders::ALL));

    f.render_widget(instructions_paragraph, layout[1]);
}
pub fn update(app: &mut App) -> Result<()> {
    if event::poll(std::time::Duration::from_millis(250))? {
        if let Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    Char('q') => app.should_quit = true,
                    Char('j') => {
                        app.start_of_window += 1;
                        app.end_of_window += 1;
                    }
                    Char('k') if app.start_of_window > 0 => {
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
pub struct App {
    pub should_quit: bool,
    pub converted_numbers: Vec<u32>,
    pub start_of_window: usize,
    pub end_of_window: usize,
}

pub fn generate_ui(config: Config) -> Result<(), Box<dyn Error>> {
    let mut t = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    let bytes_read = fs::read(config.file_path)?;
    let mut u32_numbers = Vec::new();

    add_bytes_as_u32(&bytes_read, &mut u32_numbers)?;
    let mut app = App {
        should_quit: false,
        converted_numbers: u32_numbers,
        start_of_window: 0,
        end_of_window: 36,
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
