//! VAM System Interface boundary.
//!
//! The System Interface owns VAM's interaction with the host operating system.
//! It isolates host-system operations from VAM Core so that OS details do not
//! leak into application-coordination logic.
//!
//! # Responsibility
//!
//! The System Interface owns the mechanics of interacting with the host:
//!
//! - reading host-provided process information
//! - translating OS-level failures into VAM `ErrorKind::Internal` for
//!   operations that can fail (no new error taxonomy is introduced)
//!
//! # Current scope (Goal 004)
//!
//! The current foundation performs a single host-system operation:
//!
//! - [`args`]: reading the process command-line arguments.
//!
//! The following host-system operations are intentionally **not** part of the
//! current System Interface and are deferred until genuinely required:
//!
//! - Process termination. Exit-code handling remains the responsibility of the
//!   thin process entry point (`main.rs`), per the VAM foundation contract.
//! - stdout/stderr output. Owned by the CLI presentation layer and the
//!   diagnostics boundary.
//! - Command or process spawning. Not required by the current foundation.
//! - Filesystem inspection. Not required; `Config::new` does not touch the
//!   host filesystem.
//!
//! Future operations will be added only when a real VAM requirement calls for
//! them, each placed behind this boundary.
//!
//! # Diagnostics integration
//!
//! System Interface operations may emit diagnostics for noteworthy events, but
//! only through the Goal 003 diagnostics boundary. Routine operations such as
//! reading arguments are not logged to avoid diagnostic noise, and OS details
//! are never exposed through diagnostics.

/// Reads the host process command-line arguments.
///
/// Returns every argument exactly as the host provides it, with the program
/// path as the first element, mirroring `std::env::args`. This function
/// performs no shell expansion, validation, or argument parsing; it only
/// bridges VAM to the host-provided argument vector. The caller decides which
/// elements are command arguments (for example, by skipping the program path).
///
/// This is the only host-system interaction currently owned by the System
/// Interface. It is infallible: the host always provides the arguments and
/// `std::env::args` replaces any non-UTF-8 bytes rather than yielding errors.
pub fn args() -> Vec<String> {
    std::env::args().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_returns_at_least_program_path() {
        let args = args();
        assert!(
            !args.is_empty(),
            "the host must always provide at least the program path"
        );
    }

    #[test]
    fn test_args_first_element_is_program_path() {
        let args = args();
        assert!(
            !args[0].is_empty(),
            "the program path (first argument) must be non-empty"
        );
    }

    #[test]
    fn test_args_returns_owned_strings() {
        // std::env::args replaces non-UTF-8 bytes, so args() yields valid
        // owned Strings and never panics on invalid input.
        let args: Vec<String> = args();
        assert!(!args.is_empty());
    }
}
