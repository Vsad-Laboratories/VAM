pub mod cli;
pub mod config;
pub mod error;
pub mod log;
pub mod system;

pub use error::{Error, ErrorKind, Result};

pub fn run() -> Result<()> {
    run_with_args(crate::system::args().into_iter().skip(1).collect())
}

pub fn run_with_args(args: Vec<String>) -> Result<()> {
    let _config = crate::config::Config::new()?;
    let _log = crate::log::Diagnostics::new()?;
    let command = crate::cli::parse(&args)?;
    crate::cli::dispatch(command)
}

#[cfg(test)]
mod tests {
    use crate::cli;
    use crate::config::Config;
    use crate::error::{Error, ErrorKind};
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
    fn test_unknown_command_produces_usage_error() {
        let result = cli::parse(&["unknown".to_string()]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::Usage);
        assert!(err.user_message().contains("Unknown command"));
        assert!(err.user_message().contains("help, version"));
    }

    #[test]
    fn test_too_many_args_produces_usage_error() {
        let result = cli::parse(&["help".to_string(), "extra".to_string()]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::Usage);
        assert!(err.user_message().contains("Unexpected arguments"));
    }

    #[test]
    fn test_error_propagation_from_cli_to_app() {
        let result = crate::run_with_args(vec!["unknown".to_string()]);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::Usage);
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
        let err: Error = Error::new(ErrorKind::Internal, "display test");
        let displayed = format!("{}", err);
        assert_eq!(displayed, "display test");
    }

    #[test]
    fn test_error_implements_std_error() {
        let err: Error = Error::new(ErrorKind::Internal, "source test");
        let _ = std::error::Error::source(&err);
    }

    #[test]
    fn test_error_debug_omits_source_details() {
        let source = std::io::Error::other("/etc/secret.conf");
        let err = Error::with_source(ErrorKind::Internal, "operation failed", source);
        let debug = format!("{:?}", err);
        assert!(!debug.contains("/etc/secret.conf"));
        assert!(debug.contains("present"));
    }

    #[test]
    fn test_error_with_source_preserves_kind() {
        let source = std::io::Error::other("inner");
        let err = Error::with_source(ErrorKind::Internal, "operation failed", source);
        assert_eq!(err.kind(), ErrorKind::Internal);
        assert_eq!(err.message(), "operation failed");
    }

    #[test]
    fn test_error_user_message_matches_display() {
        let err = Error::usage("user message");
        assert_eq!(err.user_message(), err.message());
    }

    #[test]
    fn test_diagnostics_log_error_integration() {
        let err = Error::new(ErrorKind::Internal, "integration test error");
        let diag = Diagnostics::new().unwrap();
        diag.log_error(&err, "integration context");
    }
}
