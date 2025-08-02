use anyhow::Context;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Padding, Paragraph},
};
use std::io;

#[derive(Debug, Clone)]
pub struct AppState {
    pub input: String,
    pub console_output: Vec<String>,
    pub cursor_position: usize,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            input: String::new(),
            console_output: vec![
                "System initialized...".to_string(),
                "Ready for commands.".to_string(),
            ],
            cursor_position: 0,
        }
    }
}

pub fn run() -> anyhow::Result<()> {
    enable_raw_mode().context("Failed to enable raw mode")?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
        .context("Failed to setup terminal")?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).context("Failed to create terminal")?;

    let mut app_state = AppState::default();
    let result = run_app(&mut terminal, &mut app_state);

    disable_raw_mode().context("Failed to disable raw mode")?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .context("Failed to restore terminal")?;
    terminal.show_cursor().context("Failed to show cursor")?;

    result
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app_state: &mut AppState,
) -> anyhow::Result<()> {
    loop {
        terminal
            .draw(|f| ui(f, app_state))
            .context("Failed to draw UI")?;

        if let Event::Key(key) = event::read().context("Failed to read input event")? {
            if should_quit(key.code) {
                break;
            }
            handle_input(key, app_state)?;
        }
    }
    Ok(())
}

fn handle_input(key: crossterm::event::KeyEvent, app_state: &mut AppState) -> anyhow::Result<()> {
    match key.code {
        KeyCode::Enter => {
            if !app_state.input.is_empty() {
                let command = app_state.input.clone();
                app_state.console_output.push(format!("> {command}"));

                let result = execute_command(&command);
                app_state.console_output.push(result);

                app_state.input.clear();
                app_state.cursor_position = 0;
            }
        }
        KeyCode::Char(c) => {
            app_state.input.insert(app_state.cursor_position, c);
            app_state.cursor_position += 1;
        }
        KeyCode::Backspace => {
            if app_state.cursor_position > 0 {
                app_state.cursor_position -= 1;
                app_state.input.remove(app_state.cursor_position);
            }
        }
        KeyCode::Left => {
            if app_state.cursor_position > 0 {
                app_state.cursor_position -= 1;
            }
        }
        KeyCode::Right => {
            if app_state.cursor_position < app_state.input.len() {
                app_state.cursor_position += 1;
            }
        }
        _ => {}
    }
    Ok(())
}

fn execute_command(command: &str) -> String {
    format!("Executed: {command}")
}

fn ui(f: &mut Frame, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(3),
            Constraint::Length(1),
        ])
        .split(f.area());

    let status_text = Line::from(vec![Span::styled(
        "Welcome to Azure AI Foundry Code!",
        Style::default().add_modifier(Modifier::BOLD),
    )]);
    let status_panel = Paragraph::new(status_text).block(
        Block::default()
            .borders(Borders::ALL)
            .padding(Padding::new(1, 0, 0, 0)),
    );
    f.render_widget(status_panel, chunks[0]);

    let console_items: Vec<ListItem> = app_state
        .console_output
        .iter()
        .map(|line| ListItem::new(Line::from(format!(" {line}"))))
        .collect();

    let console_panel = List::new(console_items);
    f.render_widget(console_panel, chunks[1]);

    let input_text = format!("> {}", app_state.input);
    let input_panel = Paragraph::new(input_text).block(
        Block::default()
            .borders(Borders::ALL)
            .padding(Padding::new(1, 0, 0, 0)),
    );
    f.render_widget(input_panel, chunks[2]);

    let instructions =
        Paragraph::new(" Press 'q' or 'Esc' to quit").style(Style::default().fg(Color::DarkGray));
    f.render_widget(instructions, chunks[3]);
}

pub fn should_quit(key_code: KeyCode) -> bool {
    matches!(key_code, KeyCode::Char('q') | KeyCode::Esc)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{Terminal, backend::TestBackend};

    #[test]
    fn test_should_quit_with_q() {
        assert!(should_quit(KeyCode::Char('q')));
    }

    #[test]
    fn test_should_quit_with_esc() {
        assert!(should_quit(KeyCode::Esc));
    }

    #[test]
    fn test_should_not_quit_with_other_keys() {
        assert!(!should_quit(KeyCode::Char('a')));
        assert!(!should_quit(KeyCode::Enter));
        assert!(!should_quit(KeyCode::Char(' ')));
        assert!(!should_quit(KeyCode::Up));
    }

    #[test]
    fn test_ui_rendering() {
        let backend = TestBackend::new(40, 10);
        let mut terminal = Terminal::new(backend).unwrap();
        let app_state = AppState::default();

        terminal.draw(|f| ui(f, &app_state)).unwrap();

        let buffer = terminal.backend().buffer();
        let buffer_content = buffer
            .content()
            .iter()
            .map(|cell| cell.symbol())
            .collect::<String>();
        assert!(buffer_content.contains("Azure AI Foundry"));
    }

    #[test]
    fn test_ui_layout_structure() {
        let backend = TestBackend::new(40, 10);
        let mut terminal = Terminal::new(backend).unwrap();
        let app_state = AppState::default();

        let result = terminal.draw(|f| ui(f, &app_state));
        assert!(result.is_ok());
    }

    #[test]
    fn test_ui_with_different_sizes() {
        let sizes = [(20, 5), (80, 24), (120, 40)];
        let app_state = AppState::default();

        for (width, height) in sizes {
            let backend = TestBackend::new(width, height);
            let mut terminal = Terminal::new(backend).unwrap();

            let result = terminal.draw(|f| ui(f, &app_state));
            assert!(
                result.is_ok(),
                "UI should render properly with size {}x{}",
                width,
                height
            );
        }
    }

    #[test]
    fn test_app_state_default() {
        let app_state = AppState::default();
        assert_eq!(app_state.input, "");
        assert_eq!(app_state.cursor_position, 0);
        assert!(!app_state.console_output.is_empty());
        assert!(
            app_state
                .console_output
                .contains(&"System initialized...".to_string())
        );
    }

    #[test]
    fn test_handle_input_char() {
        let mut app_state = AppState::default();
        let key_event = crossterm::event::KeyEvent::new(
            KeyCode::Char('a'),
            crossterm::event::KeyModifiers::NONE,
        );

        handle_input(key_event, &mut app_state).unwrap();
        assert_eq!(app_state.input, "a");
        assert_eq!(app_state.cursor_position, 1);
    }

    #[test]
    fn test_handle_input_backspace() {
        let mut app_state = AppState::default();
        app_state.input = "test".to_string();
        app_state.cursor_position = 4;

        let key_event = crossterm::event::KeyEvent::new(
            KeyCode::Backspace,
            crossterm::event::KeyModifiers::NONE,
        );

        handle_input(key_event, &mut app_state).unwrap();
        assert_eq!(app_state.input, "tes");
        assert_eq!(app_state.cursor_position, 3);
    }

    #[test]
    fn test_handle_input_cursor_movement() {
        let mut app_state = AppState::default();
        app_state.input = "test".to_string();
        app_state.cursor_position = 2;

        let left_key =
            crossterm::event::KeyEvent::new(KeyCode::Left, crossterm::event::KeyModifiers::NONE);
        handle_input(left_key, &mut app_state).unwrap();
        assert_eq!(app_state.cursor_position, 1);

        let right_key =
            crossterm::event::KeyEvent::new(KeyCode::Right, crossterm::event::KeyModifiers::NONE);
        handle_input(right_key, &mut app_state).unwrap();
        assert_eq!(app_state.cursor_position, 2);
    }

    #[test]
    fn test_execute_command() {
        let result = execute_command("test command");
        assert_eq!(result, "Executed: test command");

        let result = execute_command("");
        assert_eq!(result, "Executed: ");

        let result = execute_command("complex command with spaces");
        assert_eq!(result, "Executed: complex command with spaces");
    }

    #[test]
    fn test_handle_input_enter_with_command() {
        let mut app_state = AppState::default();
        app_state.input = "hello".to_string();
        app_state.cursor_position = 5;
        let initial_output_len = app_state.console_output.len();

        let enter_key =
            crossterm::event::KeyEvent::new(KeyCode::Enter, crossterm::event::KeyModifiers::NONE);

        handle_input(enter_key, &mut app_state).unwrap();

        // Input should be cleared
        assert_eq!(app_state.input, "");
        assert_eq!(app_state.cursor_position, 0);

        // Console output should have two new entries
        assert_eq!(app_state.console_output.len(), initial_output_len + 2);
        assert_eq!(app_state.console_output[initial_output_len], "> hello");
        assert_eq!(
            app_state.console_output[initial_output_len + 1],
            "Executed: hello"
        );
    }
}
