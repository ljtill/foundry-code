use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::AppState;
use crate::commands::execute_command;

pub fn handle_input(app: &mut AppState, key: KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Esc => {
            app.quit();
        }
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.quit();
        }
        KeyCode::Enter => {
            if !app.input.is_empty() {
                let command = app.input.clone();
                app.add_output(format!("> {command}"));
                let result = execute_command(&command);
                app.add_output(result);

                if command.trim() == "/exit" {
                    app.quit();
                }

                app.clear_input();
            }
        }
        KeyCode::Backspace => {
            app.remove_char();
        }
        KeyCode::Left => {
            app.move_cursor_left();
        }
        KeyCode::Right => {
            app.move_cursor_right();
        }
        KeyCode::Char(c) => {
            app.add_char(c);
        }
        _ => {}
    }
    Ok(())
}

pub fn should_quit(app: &AppState) -> bool {
    app.should_quit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_quit_keys() {
        // Test quit key combinations set should_quit flag
        let mut app = AppState::default();

        // Esc key should trigger quit
        let key = KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE);
        handle_input(&mut app, key).unwrap();
        assert!(app.should_quit);

        // Ctrl+C should trigger quit
        app.should_quit = false;
        let key = KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL);
        handle_input(&mut app, key).unwrap();
        assert!(app.should_quit);
    }

    #[test]
    fn test_handle_character_input() {
        // Test character input updates app state correctly
        let mut app = AppState::default();

        let key = KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE);
        handle_input(&mut app, key).unwrap();
        assert_eq!(app.input, "h");

        let key = KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE);
        handle_input(&mut app, key).unwrap();
        assert_eq!(app.input, "hi");
    }

    #[test]
    fn test_handle_backspace() {
        // Test backspace removes character and adjusts cursor
        let mut app = AppState::default();
        app.input = "hello".to_string();
        app.cursor_position = 5;

        let key = KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE);
        handle_input(&mut app, key).unwrap();
        assert_eq!(app.input, "hell");
        assert_eq!(app.cursor_position, 4);
    }

    #[test]
    fn test_handle_arrow_keys() {
        // Test arrow keys move cursor correctly
        let mut app = AppState::default();
        app.input = "hello".to_string();
        app.cursor_position = 2;

        // Left arrow moves cursor left
        let key = KeyEvent::new(KeyCode::Left, KeyModifiers::NONE);
        handle_input(&mut app, key).unwrap();
        assert_eq!(app.cursor_position, 1);

        // Right arrow moves cursor right
        let key = KeyEvent::new(KeyCode::Right, KeyModifiers::NONE);
        handle_input(&mut app, key).unwrap();
        assert_eq!(app.cursor_position, 2);
    }

    #[test]
    fn test_handle_enter_key() {
        // Test enter executes command and adds to output history
        let mut app = AppState::default();
        app.input = "/echo test".to_string();

        let initial_history_len = app.output_history.len();

        let key = KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE);
        handle_input(&mut app, key).unwrap();

        // Input should be cleared after command execution
        assert_eq!(app.input, "");
        assert_eq!(app.cursor_position, 0);

        // Command and output should be added to history
        assert!(app.output_history.len() > initial_history_len);
        assert!(
            app.output_history
                .iter()
                .any(|line| line.contains("> /echo test"))
        );
    }

    #[test]
    fn test_should_quit() {
        // Test should_quit returns correct state
        let mut app = AppState::default();
        assert!(!should_quit(&app));

        app.should_quit = true;
        assert!(should_quit(&app));
    }

    #[test]
    fn test_exit_command_quits_app() {
        // Test that typing "/exit" command quits the application
        let mut app = AppState::default();
        app.input = "/exit".to_string();

        let key = KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE);
        handle_input(&mut app, key).unwrap();

        // App should be set to quit after executing exit command
        assert!(app.should_quit);
        assert_eq!(app.input, ""); // Input should be cleared
        assert!(
            app.output_history
                .iter()
                .any(|line| line.contains("> /exit"))
        );
    }
}
