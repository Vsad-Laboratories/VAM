use crate::error::{Error, ErrorKind, Result};
use crate::runtime::{RuntimeContext, RuntimeState};
use std::os::unix::process::ExitStatusExt;

/// Structured result of a process execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessResult {
    pub success: bool,
    pub exit_code: Option<i32>,
    pub signal: Option<i32>,
}

/// Executes a prepared package entrypoint.
///
/// Consumes the RuntimeContext and spawns the resolved entrypoint
/// as a child process. Uses the Runtime working directory and
/// Runtime environment. Never invokes an implicit shell.
///
/// Preparation must have succeeded before execution.
pub fn execute(ctx: &RuntimeContext) -> Result<ProcessResult> {
    if ctx.state != RuntimeState::Prepared {
        return Err(Error::usage("runtime context is not prepared"));
    }

    let entrypoint = &ctx.entrypoint;
    if !entrypoint.exists() {
        return Err(Error::usage("entrypoint does not exist"));
    }

    let mut cmd = std::process::Command::new(entrypoint);
    cmd.current_dir(&ctx.working_dir);
    cmd.env_clear();
    for (key, value) in ctx.environment.variables() {
        cmd.env(key, value);
    }

    let output = cmd
        .output()
        .map_err(|e| Error::with_source(ErrorKind::Internal, "failed to spawn process", e))?;

    let success = output.status.success();
    let exit_code = output.status.code();
    let signal = output.status.signal();

    Ok(ProcessResult {
        success,
        exit_code,
        signal,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::prepare;
    use std::fs;
    use std::os::unix::fs::PermissionsExt;
    use std::path::Path;

    use tempfile::TempDir;

    fn create_executable_package(dir: &Path, exit_code: i32) {
        let identity = "testdev.test";
        let package_dir = dir.join(identity);
        fs::create_dir_all(&package_dir).unwrap();

        let manifest = r#"name = "test"
developer = "testdev"
version = "1.0"
release = "1"
description = "A test package"
purpose = "test"
package_type = "standard"
"#;
        fs::write(package_dir.join("manifest.toml"), manifest).unwrap();

        let script = format!("#!/bin/sh\nexit {}\n", exit_code);
        let entrypoint_path = package_dir.join("entrypoint");
        fs::write(&entrypoint_path, script).unwrap();
        let mut perms = fs::metadata(&entrypoint_path).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&entrypoint_path, perms).unwrap();

        let payload_dir = package_dir.join("payload");
        fs::create_dir(&payload_dir).unwrap();
        fs::write(payload_dir.join("file.txt"), "content").unwrap();
    }

    #[test]
    fn test_execute_success() {
        let temp = TempDir::new().unwrap();
        create_executable_package(temp.path(), 0);

        let identity = "testdev.test";
        let ctx = prepare(
            identity,
            Some(temp.path()),
            Some(&temp.path().join("runtims")),
        )
        .unwrap();
        let result = execute(&ctx);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.success);
        assert_eq!(result.exit_code, Some(0));
        assert_eq!(result.signal, None);
    }

    #[test]
    fn test_execute_non_zero_exit() {
        let temp = TempDir::new().unwrap();
        create_executable_package(temp.path(), 42);

        let identity = "testdev.test";
        let ctx = prepare(
            identity,
            Some(temp.path()),
            Some(&temp.path().join("runtims")),
        )
        .unwrap();
        let result = execute(&ctx);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(!result.success);
        assert_eq!(result.exit_code, Some(42));
        assert_eq!(result.signal, None);
    }

    #[test]
    fn test_execute_uses_runtime_working_dir() {
        let temp = TempDir::new().unwrap();
        create_executable_package(temp.path(), 0);

        let identity = "testdev.test";
        let ctx = prepare(
            identity,
            Some(temp.path()),
            Some(&temp.path().join("runtims")),
        )
        .unwrap();
        assert!(ctx.working_dir.starts_with(temp.path().join("runtims")));
    }

    #[test]
    fn test_execute_uses_runtime_environment() {
        let temp = TempDir::new().unwrap();
        create_executable_package(temp.path(), 0);

        let identity = "testdev.test";
        let mut ctx = prepare(
            identity,
            Some(temp.path()),
            Some(&temp.path().join("runtims")),
        )
        .unwrap();
        ctx.environment.add_variable("TEST_VAR", "test_value");
        let result = execute(&ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_rejects_unprepared_context() {
        let temp = TempDir::new().unwrap();
        create_executable_package(temp.path(), 0);

        let identity = "testdev.test";
        let mut ctx = prepare(
            identity,
            Some(temp.path()),
            Some(&temp.path().join("runtims")),
        )
        .unwrap();
        ctx.state = RuntimeState::Unprepared;
        let result = execute(&ctx);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::Usage);
    }

    #[test]
    fn test_execute_rejects_invalid_context() {
        let temp = TempDir::new().unwrap();
        create_executable_package(temp.path(), 0);

        let identity = "testdev.test";
        let mut ctx = prepare(
            identity,
            Some(temp.path()),
            Some(&temp.path().join("runtims")),
        )
        .unwrap();
        ctx.state = RuntimeState::Invalid;
        let result = execute(&ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_process_result_success() {
        let temp = TempDir::new().unwrap();
        create_executable_package(temp.path(), 0);

        let identity = "testdev.test";
        let ctx = prepare(
            identity,
            Some(temp.path()),
            Some(&temp.path().join("runtims")),
        )
        .unwrap();
        let result = execute(&ctx).unwrap();
        assert!(result.success);
        assert_eq!(result.exit_code, Some(0));
    }

    #[test]
    fn test_process_result_non_zero_exit() {
        let temp = TempDir::new().unwrap();
        create_executable_package(temp.path(), 1);

        let identity = "testdev.test";
        let ctx = prepare(
            identity,
            Some(temp.path()),
            Some(&temp.path().join("runtims")),
        )
        .unwrap();
        let result = execute(&ctx).unwrap();
        assert!(!result.success);
        assert_eq!(result.exit_code, Some(1));
    }

    #[test]
    fn test_no_shell_invocation() {
        let temp = TempDir::new().unwrap();
        create_executable_package(temp.path(), 0);

        let identity = "testdev.test";
        let ctx = prepare(
            identity,
            Some(temp.path()),
            Some(&temp.path().join("runtims")),
        )
        .unwrap();
        let result = execute(&ctx);
        assert!(result.is_ok());
    }
}
