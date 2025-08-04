use ratatui::layout::{Constraint, Direction, Layout, Margin, Rect};

pub fn create_main_layout(area: Rect) -> (Layout, Rect) {
    let padded_area = area.inner(Margin {
        horizontal: 0,
        vertical: 0,
    });

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Status box
            Constraint::Min(0),    // Console output (expandable)
            Constraint::Length(3), // Input box
            Constraint::Length(3), // Instructions
        ]);

    (layout, padded_area)
}

pub fn create_console_layout(_area: Rect) -> Layout {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0)])
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::layout::Rect;

    #[test]
    fn test_create_main_layout() {
        // Test main layout creates 4 sections with correct heights
        let area = Rect::new(0, 0, 80, 24);
        let (layout, padded_area) = create_main_layout(area);

        let chunks = layout.split(padded_area);
        assert_eq!(chunks.len(), 4);
        assert_eq!(chunks[0].height, 3); // Status box
        assert_eq!(chunks[2].height, 3); // Input box
        assert_eq!(chunks[3].height, 3); // Instructions
        assert_eq!(padded_area.x, 0);
        assert_eq!(padded_area.y, 0);
        assert_eq!(padded_area.width, 80);
        assert_eq!(padded_area.height, 24);
    }

    #[test]
    fn test_create_console_layout() {
        // Test console layout creates single expandable section
        let area = Rect::new(0, 0, 80, 15);
        let layout = create_console_layout(area);

        let chunks = layout.split(area);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0], area);
    }
}
