use crate::{app::App, enums::inputmodes::InputMode};
use crossterm::event::KeyCode;

pub fn handle_normal_mode_keys(app: &mut App, key: KeyCode) -> Result<(), std::io::Error> {
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
pub fn handle_editing_mode_keys(app: &mut App, key: KeyCode) -> Result<(), std::io::Error> {
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
