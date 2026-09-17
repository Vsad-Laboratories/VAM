use crate::error::{Error, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Help,
    Version,
    Run { package: String },
    Tui,
}

pub fn parse(args: &[String]) -> Result<Command> {
    match args {
        [] => Ok(Command::Help),
        [arg] => match arg.as_str() {
            "help" => Ok(Command::Help),
            "version" => Ok(Command::Version),
            "tui" => Ok(Command::Tui),
            "run" => Err(Error::usage(
                "vam run expects a package identity. Run 'vam help' for usage.",
            )),
            _ => Err(Error::usage(
                "Unknown command. Supported commands: help, version, tui, run <package>. Run 'vam help' for usage.",
            )),
        },
        [arg, package] if arg == "run" => Ok(Command::Run {
            package: package.clone(),
        }),
        [arg, _package] if arg == "run" => Err(Error::usage(
            "vam run expects exactly one package identity. Run 'vam help' for usage.",
        )),
        _ => Err(Error::usage(
            "Unexpected arguments. This command accepts at most one argument. Run 'vam help' for usage.",
        )),
    }
}

pub fn dispatch(command: Command) -> Result<()> {
    match command {
        Command::Help => {
            println!("VAM — VSAD Arch Manager");
            println!();
            println!("Usage:");
            println!("  vam           Show help");
            println!("  vam help      Show help");
            println!("  vam version   Show version");
            println!("  vam run <package>  Execute an installed package");
            println!("  vam tui       Launch the TUI");
            Ok(())
        }
        Command::Version => {
            println!("0.1.0");
            Ok(())
        }
        Command::Run { package } => {
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
        Command::Tui => {
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