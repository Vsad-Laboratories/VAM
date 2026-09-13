#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Help,
    Version,
}

pub fn parse(args: &[String]) -> Result<Command, crate::error::Error> {
    match args {
        [] => Ok(Command::Help),
        [arg] => match arg.as_str() {
            "help" => Ok(Command::Help),
            "version" => Ok(Command::Version),
            _ => Err(crate::error::Error::new(format!(
                "Unknown command: {}. Run 'vam help' for usage.",
                arg
            ))),
        },
        _ => Err(crate::error::Error::new(
            "Unexpected arguments. Run 'vam help' for usage.",
        )),
    }
}

pub fn dispatch(command: Command) -> Result<(), crate::error::Error> {
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
