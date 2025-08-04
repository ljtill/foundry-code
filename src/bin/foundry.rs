use clap::Command;
use std::env;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let _matches = Command::new("foundry")
            .version(env!("CARGO_PKG_VERSION"))
            .about("Azure AI Foundry Code")
            .get_matches();
    } else {
        foundry_terminal::run()?;
    }

    Ok(())
}
