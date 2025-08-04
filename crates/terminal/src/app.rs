#[derive(Debug, Clone)]
pub struct AppState {
    pub input: String,
    pub cursor_position: usize,
    pub output_history: Vec<String>,
    pub status_text: String,
    pub should_quit: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            input: String::new(),
            cursor_position: 0,
            output_history: Vec::new(),
            status_text: "Welcome to Azure AI Foundry Code!".to_string(),
            should_quit: false,
        }
    }
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_char(&mut self, c: char) {
        self.input.insert(self.cursor_position, c);
        self.cursor_position += 1;
    }

    pub fn remove_char(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
            self.input.remove(self.cursor_position);
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_position < self.input.len() {
            self.cursor_position += 1;
        }
    }

    pub fn clear_input(&mut self) {
        self.input.clear();
        self.cursor_position = 0;
    }

    pub fn add_output(&mut self, output: String) {
        self.output_history.push(output);
    }

    pub fn set_status(&mut self, status: String) {
        self.status_text = status;
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_default() {
        // Verify default app state has correct initial values
        let state = AppState::default();
        assert_eq!(state.input, "");
        assert_eq!(state.cursor_position, 0);
        assert_eq!(state.output_history.len(), 0);
        assert_eq!(state.status_text, "Welcome to Azure AI Foundry Code!");
        assert!(!state.should_quit);
    }

    #[test]
    fn test_app_state_add_char() {
        // Test character insertion at cursor position
        let mut state = AppState::default();
        state.add_char('h');
        state.add_char('i');
        assert_eq!(state.input, "hi");
        assert_eq!(state.cursor_position, 2);
    }

    #[test]
    fn test_app_state_remove_char() {
        // Test character removal with cursor position handling
        let mut state = AppState::default();
        state.add_char('h');
        state.add_char('i');
        state.remove_char();
        assert_eq!(state.input, "h");
        assert_eq!(state.cursor_position, 1);

        // Should not remove when at start position
        state.cursor_position = 0;
        state.remove_char();
        assert_eq!(state.input, "h");
        assert_eq!(state.cursor_position, 0);
    }

    #[test]
    fn test_app_state_cursor_movement() {
        // Test cursor movement with boundary checks
        let mut state = AppState::default();
        state.input = "hello".to_string();
        state.cursor_position = 2;

        state.move_cursor_left();
        assert_eq!(state.cursor_position, 1);

        state.move_cursor_right();
        assert_eq!(state.cursor_position, 2);

        // Should not move beyond boundaries
        state.cursor_position = 0;
        state.move_cursor_left();
        assert_eq!(state.cursor_position, 0);

        state.cursor_position = 5;
        state.move_cursor_right();
        assert_eq!(state.cursor_position, 5);
    }

    #[test]
    fn test_app_state_clear_input() {
        // Test input clearing resets both text and cursor
        let mut state = AppState::default();
        state.input = "test".to_string();
        state.cursor_position = 2;
        state.clear_input();
        assert_eq!(state.input, "");
        assert_eq!(state.cursor_position, 0);
    }

    #[test]
    fn test_app_state_add_output() {
        // Test output history accumulation
        let mut state = AppState::default();
        state.add_output("Test output".to_string());
        assert_eq!(state.output_history.len(), 1);
        assert_eq!(state.output_history[0], "Test output");
    }

    #[test]
    fn test_app_state_quit() {
        // Test quit state toggle
        let mut state = AppState::default();
        assert!(!state.should_quit);
        state.quit();
        assert!(state.should_quit);
    }
}
