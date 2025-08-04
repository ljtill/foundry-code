use anyhow::Result;
use ratatui::{Frame, Terminal, backend::Backend};

use crate::app::AppState;
use crate::ui::{
    create_console_layout, create_console_output, create_input_widget, create_instructions,
    create_main_layout, create_status_widget,
};

pub fn ui(f: &mut Frame, app: &AppState) {
    let (layout, padded_area) = create_main_layout(f.area());
    let chunks = layout.split(padded_area);

    f.render_widget(create_status_widget(app), chunks[0]);

    let console_chunks = create_console_layout(chunks[1]).split(chunks[1]);
    f.render_widget(create_console_output(app), console_chunks[0]);
    f.render_widget(create_input_widget(app), chunks[2]);
    f.render_widget(create_instructions(), chunks[3]);
}

pub fn render_ui<B: Backend>(terminal: &mut Terminal<B>, app: &AppState) -> Result<()> {
    terminal.draw(|f| ui(f, app))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::backend::TestBackend;

    #[test]
    fn test_ui_rendering() {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        let app = AppState::default();

        let result = render_ui(&mut terminal, &app);
        assert!(result.is_ok());
    }

    #[test]
    fn test_ui_function() {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        let app = AppState::default();

        let result = terminal.draw(|f| ui(f, &app));
        assert!(result.is_ok());

        // Test with populated app state
        let mut test_app = AppState::default();
        test_app.input = "test input".to_string();
        test_app.output_history = vec!["output line".to_string()];

        let result = terminal.draw(|f| ui(f, &test_app));
        assert!(result.is_ok());
    }
}
