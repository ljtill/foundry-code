use clap::Command;

pub fn run() -> anyhow::Result<()> {
    let _matches = Command::new("foundry")
        .version(env!("CARGO_PKG_VERSION"))
        .about("A foundry application")
        .get_matches();

    Ok(())
}
