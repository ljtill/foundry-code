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
    widgets::{Block, Borders, Paragraph},
};
use std::io;

pub fn run() -> anyhow::Result<()> {
    enable_raw_mode().context("Failed to enable raw mode")?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
        .context("Failed to setup terminal")?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).context("Failed to create terminal")?;

    let result = run_app(&mut terminal);

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

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> anyhow::Result<()> {
    loop {
        terminal.draw(ui).context("Failed to draw UI")?;

        if let Event::Key(key) = event::read().context("Failed to read input event")? {
            if should_quit(key.code) {
                break;
            }
        }
    }
    Ok(())
}

fn ui(f: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.area());

    let title = Paragraph::new(Line::from(vec![
        Span::styled(
            "Foundry",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" TUI Interface"),
    ]))
    .block(Block::default().borders(Borders::ALL).title("Welcome"));
    f.render_widget(title, chunks[0]);

    let instructions = Paragraph::new("Press 'q' or 'Esc' to quit")
        .block(Block::default().borders(Borders::ALL).title("Instructions"));
    f.render_widget(instructions, chunks[1]);
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

        terminal.draw(ui).unwrap();

        let buffer = terminal.backend().buffer();
        let buffer_content = buffer
            .content()
            .iter()
            .map(|cell| cell.symbol())
            .collect::<String>();
        assert!(buffer_content.contains("Foundry"));
        assert!(buffer_content.contains("Welcome"));
    }

    #[test]
    fn test_ui_layout_structure() {
        let backend = TestBackend::new(40, 10);
        let mut terminal = Terminal::new(backend).unwrap();

        let result = terminal.draw(ui);
        assert!(result.is_ok());
    }

    #[test]
    fn test_ui_with_different_sizes() {
        let sizes = [(20, 5), (80, 24), (120, 40)];

        for (width, height) in sizes {
            let backend = TestBackend::new(width, height);
            let mut terminal = Terminal::new(backend).unwrap();

            let result = terminal.draw(ui);
            assert!(
                result.is_ok(),
                "UI should render properly with size {}x{}",
                width,
                height
            );
        }
    }
}
