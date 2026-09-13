use crate::error::{Error, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Help,
    Version,
}

pub fn parse(args: &[String]) -> Result<Command> {
    match args {
        [] => Ok(Command::Help),
        [arg] => match arg.as_str() {
            "help" => Ok(Command::Help),
            "version" => Ok(Command::Version),
            _ => Err(Error::usage(
                "Unknown command. Supported commands: help, version. Run 'vam help' for usage.",
            )),
        },
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
            Ok(())
        }
        Command::Version => {
            println!("0.1.0");
            Ok(())
        }
    }
}
