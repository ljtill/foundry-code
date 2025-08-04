pub fn execute_command(command: &str) -> String {
    let trimmed = command.trim();

    if trimmed.starts_with("echo") {
        let rest = trimmed.strip_prefix("echo").unwrap_or("").trim();
        return rest.to_string();
    }

    match trimmed {
        "help" => {
            "Available commands:\n- help: Show this help message\n- echo <text>: Echo the text\n- clear: Clear the screen\n- exit: Exit the application".to_string()
        }
        "clear" => {
            "Screen cleared (simulated)".to_string()
        }
        "exit" => {
            "Goodbye!".to_string()
        }
        "" => {
            "".to_string()
        }
        _ => {
            format!("Unknown command: {trimmed}")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_help_command() {
        // Test help command returns list of available commands
        let result = execute_command("help");
        assert!(result.contains("Available commands:"));
        assert!(result.contains("help: Show this help message"));
        assert!(result.contains("exit: Exit the application"));
    }

    #[test]
    fn test_execute_echo_command() {
        // Test echo command returns text after "echo " prefix
        let result = execute_command("echo hello world");
        assert_eq!(result, "hello world");

        // With space after echo, it should return empty string
        let result = execute_command("echo ");
        assert_eq!(result, "");

        // Test with trailing whitespace
        let result = execute_command("echo trimmed text ");
        assert_eq!(result, "trimmed text");
    }

    #[test]
    fn test_execute_clear_command() {
        // Test clear command returns clear message
        let result = execute_command("clear");
        assert_eq!(result, "Screen cleared (simulated)");
    }

    #[test]
    fn test_execute_exit_command() {
        // Test exit command returns goodbye message
        let result = execute_command("exit");
        assert_eq!(result, "Goodbye!");
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
    fn test_execute_unknown_command() {
        // Test unknown commands return error message with command name
        let result = execute_command("unknown");
        assert_eq!(result, "Unknown command: unknown");

        let result = execute_command("invalid command");
        assert_eq!(result, "Unknown command: invalid command");
    }
}
