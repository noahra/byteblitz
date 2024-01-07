use crate::{format::Format, inputmodes::InputMode};

pub struct App {
    pub bytes_read: Vec<u8>,
    pub should_quit: bool,
    pub converted_binary_to_u32: Vec<u32>,
    pub converted_binary_to_ascii: Vec<char>,
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
    pub fn move_cursor_left(&mut self) {
        let cursor_position = self.cursor_position;
        let cursor_moved_left = cursor_position.saturating_sub(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_position.saturating_add(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_right);
    }

    pub fn enter_char(&mut self, new_char: char) {
        // Check if the character is numeric and not a space or newline
        if new_char.is_numeric() && new_char != ' ' && new_char != '\n' {
            self.input.insert(self.cursor_position, new_char);
            self.move_cursor_right();
        }
    }

    pub fn delete_char(&mut self) {
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

    pub fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.len())
    }

    pub fn reset_cursor(&mut self) {
        self.cursor_position = 0;
    }

    pub fn submit_message(&mut self) {
        if let Ok(num) = self.input.trim().parse::<usize>() {
            if num > 0 {
                let end = num.checked_add(30).unwrap_or(usize::MAX);
                let start = num.saturating_sub(1);
    
                if end > self.max_length {
                    self.start_of_window = self.max_length.saturating_sub(30);
                    self.end_of_window = self.max_length+1;
                } else {
                    self.start_of_window = start;
                    self.end_of_window = end;
                }
            }
        }
        self.input.clear();
        self.reset_cursor();
    }
    
}