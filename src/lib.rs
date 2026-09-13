pub mod cli;
pub mod config;
pub mod error;
pub mod log;

pub fn run() -> Result<(), crate::error::Error> {
    run_with_args(std::env::args().skip(1).collect())
}

pub fn run_with_args(args: Vec<String>) -> Result<(), crate::error::Error> {
    let _config = crate::config::Config::new()?;
    let _log = crate::log::Diagnostics::new()?;
    let command = crate::cli::parse(&args)?;
    crate::cli::dispatch(command)
}

#[cfg(test)]
mod tests {
    use crate::cli;
    use crate::config::Config;
    use crate::error::Error;
    use crate::log::Diagnostics;

    #[test]
    fn test_application_initialization_succeeds() {
        let result = crate::run_with_args(vec!["version".to_string()]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_cli_dispatch_help() {
        let result = cli::dispatch(cli::Command::Help);
        assert!(result.is_ok());
    }

    #[test]
    fn test_cli_dispatch_version() {
        let result = cli::dispatch(cli::Command::Version);
        assert!(result.is_ok());
    }

    #[test]
    fn test_version_behavior() {
        let cmd = cli::parse(&["version".to_string()]).unwrap();
        assert_eq!(cmd, cli::Command::Version);
    }

    #[test]
    fn test_help_behavior() {
        let cmd = cli::parse(&["help".to_string()]).unwrap();
        assert_eq!(cmd, cli::Command::Help);
    }

    #[test]
    fn test_cli_dispatch_empty_args_shows_help() {
        let cmd = cli::parse(&[]).unwrap();
        assert_eq!(cmd, cli::Command::Help);
    }

    #[test]
    fn test_error_propagation() {
        let err = Error::new("test error message");
        assert!(err.to_string().contains("test error message"));
    }

    #[test]
    fn test_error_from_unknown_command() {
        let result = cli::parse(&["unknown".to_string()]);
        assert!(result.is_err());
    }

    #[test]
    fn test_error_from_too_many_args() {
        let result = cli::parse(&["help".to_string(), "extra".to_string()]);
        assert!(result.is_err());
    }

    #[test]
    fn test_configuration_boundary_initializes() {
        let config = Config::new();
        assert!(config.is_ok());
    }

    #[test]
    fn test_diagnostic_boundary_initializes() {
        let diag = Diagnostics::new();
        assert!(diag.is_ok());
    }

    #[test]
    fn test_error_implements_display() {
        let err: Error = Error::new("display test");
        let displayed = format!("{}", err);
        assert_eq!(displayed, "display test");
    }

    #[test]
    fn test_error_implements_std_error() {
        let err: Error = Error::new("source test");
        let _ = std::error::Error::source(&err);
    }
}
