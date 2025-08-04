pub mod app;
pub mod commands;
pub mod events;
pub mod terminal;
pub mod ui;

pub use app::AppState;
pub use events::{handle_input, should_quit};
pub use terminal::{TerminalType, restore_terminal, setup_terminal};
pub use ui::render_ui;

use anyhow::Context;
use crossterm::event::{self, Event};

pub fn run() -> anyhow::Result<()> {
    let mut terminal = setup_terminal()?;
    let mut app_state = AppState::default();
    let result = run_app(&mut terminal, &mut app_state);
    restore_terminal(&mut terminal)?;
    result
}

fn run_app(terminal: &mut TerminalType, app_state: &mut AppState) -> anyhow::Result<()> {
    loop {
        render_ui(terminal, app_state)?;

        if let Event::Key(key) = event::read().context("Failed to read input event")? {
            handle_input(app_state, key)?;
            if should_quit(app_state) {
                break;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
    use ratatui::{Terminal, backend::TestBackend};

    #[test]
    fn test_app_state_integration() {
        // Verify app state initializes with correct default values
        let app = AppState::default();
        assert_eq!(app.input, "");
        assert_eq!(app.cursor_position, 0);
        assert!(!app.output_history.is_empty());
        assert_eq!(app.status_text, "🚀 Welcome to Azure AI Foundry Code!");
        assert!(!app.should_quit);
    }

    #[test]
    fn test_event_handling_integration() {
        // Test complete event handling flow from key input to state changes
        let mut app = AppState::default();

        // Character input should update input string
        let key = KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE);
        handle_input(&mut app, key).unwrap();
        assert_eq!(app.input, "h");

        // Esc key should trigger quit state
        let key = KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE);
        handle_input(&mut app, key).unwrap();
        assert!(should_quit(&app));
    }

    #[test]
    fn test_ui_rendering_integration() {
        // Verify UI renders without errors on test backend
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        let app = AppState::default();

        let result = render_ui(&mut terminal, &app);
        assert!(result.is_ok());
    }

    #[test]
    fn test_command_execution_integration() {
        // Test command execution returns expected output
        use crate::commands::execute_command;

        let result = execute_command("/help");
        assert!(result.contains("Available System Commands"));

        let result = execute_command("How are you?");
        assert_eq!(result, "You said: How are you?");
    }
}
