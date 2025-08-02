use std::env;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    // Check if any CLI flags are provided
    if args.len() > 1 {
        // Delegate to CLI handler
        foundry_cli::run()?;
    } else {
        // Default to TUI
        foundry_tui::run()?;
    }

    Ok(())
}
