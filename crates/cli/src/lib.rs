use clap::Command;

pub fn run() -> anyhow::Result<()> {
    let _matches = build_cli().get_matches();
    Ok(())
}

pub fn build_cli() -> Command {
    Command::new("foundry")
        .version(env!("CARGO_PKG_VERSION"))
        .about("A foundry application")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_creation() {
        let cmd = build_cli();
        assert_eq!(cmd.get_name(), "foundry");
        assert!(cmd.get_about().is_some());
        assert!(cmd.get_version().is_some());
    }

    #[test]
    fn test_cli_version() {
        let cmd = build_cli();
        let version = cmd.get_version().unwrap();
        assert!(!version.is_empty());
    }

    #[test]
    fn test_cli_help_flag() {
        let cmd = build_cli();
        let matches = cmd.try_get_matches_from(vec!["foundry", "--help"]);
        assert!(matches.is_err());

        let err = matches.unwrap_err();
        assert_eq!(err.kind(), clap::error::ErrorKind::DisplayHelp);
    }

    #[test]
    fn test_cli_version_flag() {
        let cmd = build_cli();
        let matches = cmd.try_get_matches_from(vec!["foundry", "--version"]);
        assert!(matches.is_err());

        let err = matches.unwrap_err();
        assert_eq!(err.kind(), clap::error::ErrorKind::DisplayVersion);
    }
}
