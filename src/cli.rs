use crate::error::{Error, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Help,
    Version,
    Run { package: String },
}

pub fn parse(args: &[String]) -> Result<Command> {
    match args {
        [] => Ok(Command::Help),
        [arg] => match arg.as_str() {
            "help" => Ok(Command::Help),
            "version" => Ok(Command::Version),
            _ => Err(Error::usage(
                "Unknown command. Supported commands: help, version, run <package>. Run 'vam help' for usage.",
            )),
        },
        [arg, package] if arg == "run" => Ok(Command::Run {
            package: package.clone(),
        }),
        [arg, _package, ..] if arg == "run" => Err(Error::usage(
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
    }
}
