use crate::{
    config::Config,
    conversions::{add_bytes_as_u32, convert_bytes_to_utf8},
    format::Format,
    inputmodes::InputMode,
};
use anyhow::Result;
use crossterm::{
    event::{
        self,
        KeyCode::{self},
    },
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

pub fn update(app: &mut App) -> Result<()> {
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
fn handle_normal_mode_keys(app: &mut App, key: KeyCode) -> Result<()> {
    match key {
        KeyCode::Char('q') => {
            app.should_quit = true;
        }
        KeyCode::Char('j') => {
            app.start_of_window += 1;
            app.end_of_window += 1;
        }
        KeyCode::Char('k') if app.start_of_window > 0 => {
            app.start_of_window -= 1;
            app.end_of_window -= 1;
        }
        KeyCode::Char('h') if app.format_list_index > 0 => {
            app.format_list_index -= 1;
            app.current_format = app.format_list[app.format_list_index];
            app.start_of_window = 0;
            app.end_of_window = 30;
        }
        KeyCode::Char('l') if app.format_list_index < app.format_list.len() - 1 => {
            app.format_list_index += 1;
            app.current_format = app.format_list[app.format_list_index];
            app.start_of_window = 0;
            app.end_of_window = 30;
        }
        KeyCode::Char('e') => {
            app.input_mode = InputMode::Editing;
        }
        KeyCode::Char('n') => {
            return Ok(());
        }
        _ => {}
    }
    Ok(())
}

// Function to handle key presses in editing mode
fn handle_editing_mode_keys(app: &mut App, key: KeyCode) -> Result<()> {
    match key {
        KeyCode::Enter => {
            if !app.input.trim().is_empty() {
                app.submit_message();
            }
        }
        KeyCode::Char(to_insert) => {
            app.enter_char(to_insert);
        }
        KeyCode::Backspace => {
            app.delete_char();
        }
        KeyCode::Left => {
            app.move_cursor_left();
        }
        KeyCode::Right => {
            app.move_cursor_right();
        }
        KeyCode::Esc => {
            app.input_mode = InputMode::Normal;
        }
        _ => {}
    }
    Ok(())
}
pub struct App {
    pub bytes_read: Vec<u8>,
    pub should_quit: bool,
    pub converted_numbers: Vec<u32>,
    pub converted_binary_to_utf8: Vec<char>,
    pub start_of_window: usize,
    pub end_of_window: usize,
    pub current_format: Format,
    pub format_list_index: usize,
    pub format_list: Vec<Format>,
    pub input: String,
    input_mode: InputMode,
    pub cursor_position: usize,
    pub max_length: usize,
}
impl App {
    fn move_cursor_left(&mut self) {
        let cursor_position = self.cursor_position;
        let cursor_moved_left = cursor_position.saturating_sub(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_position.saturating_add(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        // Check if the character is numeric and not a space or newline
        if new_char.is_numeric() && new_char != ' ' && new_char != '\n' {
            self.input.insert(self.cursor_position, new_char);
            self.move_cursor_right();
        }
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.cursor_position != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.cursor_position;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.len())
    }

    fn reset_cursor(&mut self) {
        self.cursor_position = 0;
    }

    fn submit_message(&mut self) {
        // TODO - proper error handling
        if self.input.parse::<usize>().unwrap() > 0
            && self.input.parse::<usize>().unwrap() < self.max_length - 40
        {
            self.start_of_window = self.input.parse::<usize>().unwrap() - 1;
            self.end_of_window = self.input.parse::<usize>().unwrap() + 36;
        }
        self.input.clear();
        self.reset_cursor();
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
