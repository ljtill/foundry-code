use anyhow::{Context, Result};
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io::{self, Stdout};

pub type TerminalType = Terminal<CrosstermBackend<Stdout>>;

pub fn setup_terminal() -> Result<TerminalType> {
    enable_raw_mode().context("Failed to enable raw mode")?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).context("Failed to enter alternate screen")?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend).context("Failed to create terminal")?;
    Ok(terminal)
}

pub fn restore_terminal(terminal: &mut TerminalType) -> Result<()> {
    disable_raw_mode().context("Failed to disable raw mode")?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)
        .context("Failed to leave alternate screen")?;
    terminal.show_cursor().context("Failed to show cursor")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terminal_functions_exist() {
        // Verify functions exist and have correct signatures
        let _setup_fn: fn() -> Result<TerminalType> = setup_terminal;
        let _restore_fn: fn(&mut TerminalType) -> Result<()> = restore_terminal;
        let _terminal_type_size = std::mem::size_of::<TerminalType>();

        assert!(_terminal_type_size > 0);
    }
}
