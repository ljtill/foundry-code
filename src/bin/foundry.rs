use std::env;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        foundry_cli::run()?;
    } else {
        foundry_tui::run()?;
    }

    Ok(())
}
