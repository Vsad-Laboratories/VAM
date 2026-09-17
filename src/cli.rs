use crate::error::{Error, Result};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "vam")]
#[command(about = "VAM — VSAD Arch Manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug, Clone, PartialEq, Eq)]
pub enum Command {
    /// Show version
    Version,
    /// Execute an installed package
    Run { package: String },
    /// Launch the TUI
    Tui,
}

pub fn parse(args: &[String]) -> Result<Option<Command>> {
    let cli = Cli::try_parse_from(args).map_err(|e| {
        Error::usage(e.to_string())
    })?;
    Ok(cli.command)
}

pub fn dispatch(command: Option<Command>) -> Result<()> {
    match command {
        None => {
            // No command provided — clap will show help when run with --help or -h.
            // For the default case, show help manually.
            println!("VAM — VSAD Arch Manager");
            println!();
            println!("Usage:");
            println!("  vam           Show help");
            println!("  vam version   Show version");
            println!("  vam run <package>  Execute an installed package");
            println!("  vam tui       Launch the TUI");
            Ok(())
        }
        Some(Command::Version) => {
            println!("0.1.0");
            Ok(())
        }
        Some(Command::Run { package }) => {
            let identity = package;
            let ctx = crate::runtime::prepare(&identity, None, None)?;
            let result = crate::executor::execute(&ctx)?;
            if result.success {
                println!("{} exited successfully", identity);
            } else {
                println!("{} exited with code {:?}", identity, result.exit_code);
            }
            Ok(())
        }
        Some(Command::Tui) => {
            crate::tui::run()?;
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_version() {
        let cmd = parse(&["version".to_string()]).unwrap();
        assert!(matches!(cmd, Some(Command::Version)));
    }

    #[test]
    fn test_parse_run() {
        let cmd = parse(&["run".to_string(), "testdev.test".to_string()]).unwrap();
        assert!(matches!(cmd, Some(Command::Run { package }) if package == "testdev.test"));
    }

    #[test]
    fn test_parse_tui() {
        let cmd = parse(&["tui".to_string()]).unwrap();
        assert!(matches!(cmd, Some(Command::Tui)));
    }

    #[test]
    fn test_parse_empty_defaults_to_none() {
        let cmd = parse(&[]).unwrap();
        assert!(cmd.is_none());
    }
}