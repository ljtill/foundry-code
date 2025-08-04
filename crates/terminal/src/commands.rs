pub fn get_help_message() -> Vec<&'static str> {
    vec![
        "Available System Commands (prefix with /):",
        "  /help     - Show this help message",
        "  /clear    - Clear the screen",
        "  /exit     - Exit the application",
        "  /login    - Login to system (coming soon)",
        "  /logout   - Logout from system (coming soon)",
        "",
        "💡 Tips:",
        "  • Use arrow keys (←→) to move cursor",
        "  • Press Enter to execute commands",
        "  • Press Esc to exit anytime",
        "",
        "Type any message or use system commands above to get started!",
    ]
}

pub fn execute_command(input: &str) -> String {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return String::new();
    }

    if trimmed.starts_with('/') {
        execute_system_command(trimmed)
    } else {
        handle_user_input(trimmed)
    }
}

fn execute_system_command(command: &str) -> String {
    let cmd = command.strip_prefix('/').unwrap_or(command).trim();

    match cmd {
        "help" => get_help_message().join("\n"),
        "clear" => "Screen cleared (simulated)".to_string(),
        "exit" => "Goodbye!".to_string(),
        "login" => "Login functionality not yet implemented.".to_string(),
        "logout" => "Logout functionality not yet implemented.".to_string(),
        _ => {
            format!("Unknown system command: /{cmd}\nType /help for available commands.")
        }
    }
}

/// Handles user input that is not a system command.
///
/// Currently, this function simply echoes back the user input with a prefix.
/// In the future, this function will be extended to integrate with AI/chat functionality,
/// allowing for conversational responses, context-aware interactions, and integration
/// with AI models or chat services.
fn handle_user_input(input: &str) -> String {
    format!("You said: {input}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_system_help_command() {
        // Test /help command returns list of available system commands
        let result = execute_command("/help");
        assert!(result.contains("Available System Commands"));
        assert!(result.contains("/help     - Show this help message"));
        assert!(result.contains("/exit     - Exit the application"));
        assert!(result.contains("/login    - Login to system"));
        assert!(result.contains("/logout   - Logout from system"));
    }

    #[test]
    fn test_execute_system_clear_command() {
        // Test /clear command returns clear message
        let result = execute_command("/clear");
        assert_eq!(result, "Screen cleared (simulated)");
    }

    #[test]
    fn test_execute_system_exit_command() {
        // Test /exit command returns goodbye message
        let result = execute_command("/exit");
        assert_eq!(result, "Goodbye!");
    }

    #[test]
    fn test_execute_system_login_command() {
        // Test /login command returns not implemented message
        let result = execute_command("/login");
        assert_eq!(result, "Login functionality not yet implemented.");
    }

    #[test]
    fn test_execute_system_logout_command() {
        // Test /logout command returns not implemented message
        let result = execute_command("/logout");
        assert_eq!(result, "Logout functionality not yet implemented.");
    }

    #[test]
    fn test_execute_empty_command() {
        // Test empty and whitespace-only commands return empty string
        let result = execute_command("");
        assert_eq!(result, "");

        let result = execute_command("   ");
        assert_eq!(result, "");
    }

    #[test]
    fn test_execute_unknown_system_command() {
        // Test unknown system commands return error message with command name
        let result = execute_command("/unknown");
        assert!(result.contains("Unknown system command: /unknown"));
        assert!(result.contains("Type /help for available commands"));

        let result = execute_command("/invalid command");
        assert!(result.contains("Unknown system command: /invalid command"));
    }

    #[test]
    fn test_execute_user_input() {
        // Test regular user input is handled as questions/general input
        let result = execute_command("How are you?");
        assert_eq!(result, "You said: How are you?");

        let result = execute_command("What is the weather today?");
        assert_eq!(result, "You said: What is the weather today?");

        let result = execute_command("hello world");
        assert_eq!(result, "You said: hello world");
    }

    #[test]
    fn test_command_routing() {
        // Test that commands are routed correctly based on prefix

        // System commands start with /
        let result = execute_command("/help");
        assert!(result.contains("System Commands"));

        // Regular input doesn't start with /
        let result = execute_command("help");
        assert_eq!(result, "You said: help");

        // Edge case: just the / character
        let result = execute_command("/");
        assert!(result.contains("Unknown system command"));
    }
}
