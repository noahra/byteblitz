use crossterm::event::{self, KeyCode};
use crate::{inputmodes::InputMode, format::Format};

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
fn handle_normal_mode_keys(app: &mut App, key: KeyCode) -> Result<(), std::io::Error> {
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
fn handle_editing_mode_keys(app: &mut App, key: KeyCode) -> Result<(), std::io::Error> {
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
    pub input_mode: InputMode,
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