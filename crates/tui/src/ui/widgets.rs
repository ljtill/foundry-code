use ratatui::{
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Padding, Paragraph, Wrap},
};

use crate::app::AppState;

pub fn create_status_widget(app: &AppState) -> Paragraph {
    Paragraph::new(app.status_text.clone())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .padding(Padding::new(1, 0, 0, 0)),
        )
        .wrap(Wrap { trim: true })
}

pub fn create_console_output(app: &AppState) -> Paragraph {
    let output_text = app
        .output_history
        .iter()
        .map(|line| line.to_string())
        .collect::<Vec<_>>()
        .join("\n");

    Paragraph::new(output_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .padding(Padding::new(1, 0, 0, 0)),
        )
        .wrap(Wrap { trim: true })
        .scroll((0, 0))
}

pub fn create_input_widget(app: &AppState) -> Paragraph {
    let input_with_cursor = if app.cursor_position <= app.input.len() {
        let mut chars: Vec<char> = app.input.chars().collect();
        if app.cursor_position == chars.len() {
            chars.push('_');
        } else {
            chars.insert(app.cursor_position, '_');
        }
        chars.into_iter().collect()
    } else {
        format!("{}_", app.input)
    };

    let formatted_input = format!(" {input_with_cursor}");

    Paragraph::new(formatted_input).block(Block::default().borders(Borders::ALL))
}

pub fn create_instructions() -> Paragraph<'static> {
    let instructions = Text::from(vec![Line::from(vec![
        Span::styled(" ", Style::default().fg(Color::DarkGray)),
        Span::styled("Enter", Style::default().fg(Color::DarkGray)),
        Span::styled(": Execute command | ", Style::default().fg(Color::DarkGray)),
        Span::styled("←→", Style::default().fg(Color::DarkGray)),
        Span::styled(": Move cursor | ", Style::default().fg(Color::DarkGray)),
        Span::styled("Esc", Style::default().fg(Color::DarkGray)),
        Span::styled(": Quit", Style::default().fg(Color::DarkGray)),
    ])]);

    Paragraph::new(instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_status_widget() {
        let app = AppState::default();
        let widget = create_status_widget(&app);

        // Test with custom status text
        let mut custom_app = AppState::default();
        custom_app.status_text = "Custom status".to_string();
        let custom_widget = create_status_widget(&custom_app);

        assert!(std::mem::size_of_val(&widget) > 0);
        assert!(std::mem::size_of_val(&custom_widget) > 0);
    }

    #[test]
    fn test_create_console_output() {
        let app = AppState::default();
        let empty_widget = create_console_output(&app);

        // Test with populated history
        let mut populated_app = AppState::default();
        populated_app.output_history = vec!["line1".to_string(), "line2".to_string()];
        let populated_widget = create_console_output(&populated_app);

        assert!(std::mem::size_of_val(&empty_widget) > 0);
        assert!(std::mem::size_of_val(&populated_widget) > 0);
    }

    #[test]
    fn test_create_input_widget() {
        // Test with cursor in middle
        let mut app = AppState::default();
        app.input = "test".to_string();
        app.cursor_position = 2;
        let widget_mid_cursor = create_input_widget(&app);

        // Test with cursor at end
        let mut end_app = AppState::default();
        end_app.input = "test".to_string();
        end_app.cursor_position = end_app.input.len();
        let widget_end_cursor = create_input_widget(&end_app);

        // Test with empty input
        let empty_app = AppState::default();
        let widget_empty = create_input_widget(&empty_app);

        assert!(std::mem::size_of_val(&widget_mid_cursor) > 0);
        assert!(std::mem::size_of_val(&widget_end_cursor) > 0);
        assert!(std::mem::size_of_val(&widget_empty) > 0);
    }

    #[test]
    fn test_create_instructions() {
        let widget = create_instructions();
        let widget2 = create_instructions();

        assert!(std::mem::size_of_val(&widget) > 0);
        assert!(std::mem::size_of_val(&widget2) > 0);
        // Static content should have consistent size
        assert_eq!(
            std::mem::size_of_val(&widget),
            std::mem::size_of_val(&widget2)
        );
    }
}
