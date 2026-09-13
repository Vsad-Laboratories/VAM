use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Debug => write!(f, "debug"),
            Self::Info => write!(f, "info"),
            Self::Warn => write!(f, "warning"),
            Self::Error => write!(f, "error"),
        }
    }
}

pub struct Diagnostics;

impl Diagnostics {
    pub fn new() -> crate::Result<Self> {
        Ok(Self)
    }

    pub fn debug(&self, message: impl fmt::Display) {
        self.log(LogLevel::Debug, message);
    }

    pub fn info(&self, message: impl fmt::Display) {
        self.log(LogLevel::Info, message);
    }

    pub fn warn(&self, message: impl fmt::Display) {
        self.log(LogLevel::Warn, message);
    }

    pub fn error(&self, message: impl fmt::Display) {
        self.log(LogLevel::Error, message);
    }

    pub fn log_error(&self, error: &crate::error::Error, context: impl fmt::Display) {
        self.error(format!("{}: {}", context, error.user_message()));
    }

    fn log(&self, level: LogLevel, message: impl fmt::Display) {
        let entry = format!("[{}] {}", level, message);
        match level {
            LogLevel::Error | LogLevel::Warn => eprintln!("{}", entry),
            _ => println!("{}", entry),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::{Error, ErrorKind};

    #[test]
    fn test_log_level_display() {
        assert_eq!(LogLevel::Debug.to_string(), "debug");
        assert_eq!(LogLevel::Info.to_string(), "info");
        assert_eq!(LogLevel::Warn.to_string(), "warning");
        assert_eq!(LogLevel::Error.to_string(), "error");
    }

    #[test]
    fn test_diagnostics_new_succeeds() {
        let diag = Diagnostics::new();
        assert!(diag.is_ok());
    }

    #[test]
    fn test_log_methods_do_not_panic() {
        let diag = Diagnostics::new().unwrap();
        diag.debug("debug message");
        diag.info("info message");
        diag.warn("warn message");
        diag.error("error message");
    }

    #[test]
    fn test_log_error_does_not_expose_source_details() {
        let source = std::io::Error::other("/etc/secret.conf");
        let err = Error::with_source(ErrorKind::Internal, "operation failed", source);
        let diag = Diagnostics::new().unwrap();

        assert!(!err.user_message().contains("/etc/secret.conf"));
        diag.log_error(&err, "test context");
    }
}
