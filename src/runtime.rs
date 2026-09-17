use crate::error::{Error, ErrorKind, Result};
use crate::package::{validate_package_path, Manifest};
use std::path::{Path, PathBuf};

/// Runtime state for a prepared package.
///
/// Does not imply the package is running.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeState {
    Unprepared,
    Prepared,
    Invalid,
}

/// Explicit runtime environment representation.
///
/// Does not automatically inherit the host environment.
/// Future execution can consume this representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeEnvironment {
    variables: Vec<(String, String)>,
}

/// The runtime context for a prepared installed package.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeContext {
    pub identity: String,
    pub package_root: PathBuf,
    pub manifest: Manifest,
    pub entrypoint: PathBuf,
    pub working_dir: PathBuf,
    pub environment: RuntimeEnvironment,
    pub state: RuntimeState,
}

/// Canonical installation root for packages.
pub const INSTALL_ROOT: &str = "/usr/lib/vam/packages";

/// Canonical runtime data root.
pub const RUNTIME_ROOT: &str = "/usr/lib/vam/runtims";

/// Prepares an installed package for future execution.
///
/// Locates the installed package at `<install_root>/<identity>`,
/// loads its manifest, validates required runtime information,
/// resolves the entrypoint, establishes the working directory,
/// and returns a prepared [`RuntimeContext`].
///
/// Preparation never executes package code.
pub fn prepare(
    identity: &str,
    install_root: Option<&Path>,
    runtime_root: Option<&Path>,
) -> Result<RuntimeContext> {
    let install_root = install_root.unwrap_or_else(|| Path::new(INSTALL_ROOT));
    let runtime_root = runtime_root.unwrap_or_else(|| Path::new(RUNTIME_ROOT));

    let package_root = install_root.join(identity);

    if !package_root.is_dir() {
        return Err(Error::usage("installed package not found"));
    }

    let manifest_path = package_root.join("manifest.toml");
    let manifest_toml = std::fs::read_to_string(&manifest_path)
        .map_err(|e| Error::with_source(ErrorKind::Internal, "failed to read manifest", e))?;
    let info = crate::package::container::deserialize_toml(&manifest_toml)?;
    let manifest = Manifest {
        name: info.name,
        developer: info.developer,
        version: info.version,
        release: info.release,
        description: info.description,
        purpose: info.purpose,
        package_type: info.package_type,
    };
    manifest.validate()?;

    let entrypoint = resolve_entrypoint(&package_root, "entrypoint")?;

    let working_dir = runtime_root.join(identity).join("work");

    let environment = RuntimeEnvironment::new();

    Ok(RuntimeContext {
        identity: identity.to_string(),
        package_root,
        manifest,
        entrypoint,
        working_dir,
        environment,
        state: RuntimeState::Prepared,
    })
}

/// Resolves the package entrypoint from the installed package representation.
///
/// Verifies that:
/// - the entrypoint path is package-relative (not absolute)
/// - the entrypoint contains no `..` traversal
/// - the resolved path stays inside the package root
/// - the entrypoint exists as a file
///
/// Does NOT execute the entrypoint.
pub fn resolve_entrypoint(package_root: &Path, entrypoint: &str) -> Result<PathBuf> {
    validate_package_path(entrypoint)?;

    let entrypoint_path = Path::new(entrypoint);
    if entrypoint_path.is_absolute() {
        return Err(Error::usage("entrypoint must not be an absolute path"));
    }

    let resolved = package_root.join(entrypoint_path);

    if !resolved.exists() {
        return Err(Error::usage("entrypoint does not exist"));
    }
    if !resolved.is_file() {
        return Err(Error::usage("entrypoint is not a regular file"));
    }

    let canonical_root = package_root.canonicalize().map_err(|e| {
        Error::with_source(
            ErrorKind::Internal,
            "failed to canonicalize package root",
            e,
        )
    })?;
    let canonical_resolved = resolved.canonicalize().map_err(|e| {
        Error::with_source(ErrorKind::Internal, "failed to canonicalize entrypoint", e)
    })?;

    if !canonical_resolved.starts_with(&canonical_root) {
        return Err(Error::usage("entrypoint escapes the package root"));
    }

    Ok(canonical_resolved)
}

impl RuntimeEnvironment {
    pub fn new() -> Self {
        Self {
            variables: Vec::new(),
        }
    }

    pub fn add_variable(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.variables.push((key.into(), value.into()));
    }

    pub fn variables(&self) -> &[(String, String)] {
        &self.variables
    }
}

impl Default for RuntimeEnvironment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    use tempfile::TempDir;

    fn create_installed_package(dir: &Path) {
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

        let entrypoint_dir = package_dir.join("entrypoint");
        fs::write(&entrypoint_dir, "#!/bin/sh\necho hello").unwrap();

        let payload_dir = package_dir.join("payload");
        fs::create_dir(&payload_dir).unwrap();
        fs::write(payload_dir.join("file.txt"), "content").unwrap();
    }

    #[test]
    fn test_prepare_success() {
        let temp = TempDir::new().unwrap();
        create_installed_package(temp.path());

        let identity = "testdev.test";
        let ctx = prepare(
            identity,
            Some(temp.path()),
            Some(&temp.path().join("runtims")),
        );
        assert!(ctx.is_ok());
        let ctx = ctx.unwrap();
        assert_eq!(ctx.identity, identity);
        assert_eq!(ctx.manifest.name, "test");
        assert_eq!(ctx.manifest.developer, "testdev");
        assert_eq!(ctx.state, RuntimeState::Prepared);
    }

    #[test]
    fn test_prepare_missing_package() {
        let temp = TempDir::new().unwrap();
        let result = prepare(
            "nonexistent.pkg",
            Some(temp.path()),
            Some(&temp.path().join("runtims")),
        );
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::Usage);
    }

    #[test]
    fn test_prepare_missing_manifest() {
        let temp = TempDir::new().unwrap();
        let identity = "testdev.test";
        let package_dir = temp.path().join(identity);
        fs::create_dir_all(&package_dir).unwrap();

        let result = prepare(
            identity,
            Some(temp.path()),
            Some(&temp.path().join("runtims")),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_prepare_missing_entrypoint() {
        let temp = TempDir::new().unwrap();
        let identity = "testdev.test";
        let package_dir = temp.path().join(identity);
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

        let result = prepare(
            identity,
            Some(temp.path()),
            Some(&temp.path().join("runtims")),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_prepare_invalid_manifest() {
        let temp = TempDir::new().unwrap();
        let identity = "testdev.test";
        let package_dir = temp.path().join(identity);
        fs::create_dir_all(&package_dir).unwrap();

        fs::write(package_dir.join("manifest.toml"), "name = \"\"\ndeveloper = \"testdev\"\nversion = \"1.0\"\nrelease = \"1\"\ndescription = \"desc\"\npurpose = \"purp\"\npackage_type = \"standard\"\n").unwrap();
        fs::write(package_dir.join("entrypoint"), "#!/bin/sh\necho hello").unwrap();

        let result = prepare(
            identity,
            Some(temp.path()),
            Some(&temp.path().join("runtims")),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_resolve_entrypoint_success() {
        let temp = TempDir::new().unwrap();
        create_installed_package(temp.path());

        let package_root = temp.path().join("testdev.test");
        let result = resolve_entrypoint(&package_root, "entrypoint");
        assert!(result.is_ok());
        let resolved = result.unwrap();
        assert!(resolved.is_absolute());
        assert!(resolved.ends_with("entrypoint"));
    }

    #[test]
    fn test_resolve_entrypoint_rejects_absolute() {
        let temp = TempDir::new().unwrap();
        create_installed_package(temp.path());

        let package_root = temp.path().join("testdev.test");
        let result = resolve_entrypoint(&package_root, "/etc/passwd");
        assert!(result.is_err());
    }

    #[test]
    fn test_resolve_entrypoint_rejects_parent_dir() {
        let temp = TempDir::new().unwrap();
        create_installed_package(temp.path());

        let package_root = temp.path().join("testdev.test");
        let result = resolve_entrypoint(&package_root, "../etc/passwd");
        assert!(result.is_err());
    }

    #[test]
    fn test_resolve_entrypoint_rejects_missing() {
        let temp = TempDir::new().unwrap();
        create_installed_package(temp.path());

        let package_root = temp.path().join("testdev.test");
        let result = resolve_entrypoint(&package_root, "nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_resolve_entrypoint_rejects_directory() {
        let temp = TempDir::new().unwrap();
        create_installed_package(temp.path());

        let package_root = temp.path().join("testdev.test");
        let result = resolve_entrypoint(&package_root, "payload");
        assert!(result.is_err());
    }

    #[test]
    fn test_resolve_entrypoint_rejects_escape() {
        let temp = TempDir::new().unwrap();
        create_installed_package(temp.path());

        let package_root = temp.path().join("testdev.test");
        let result = resolve_entrypoint(&package_root, "../../etc/passwd");
        assert!(result.is_err());
    }

    #[test]
    fn test_environment_no_host_inheritance() {
        let env = RuntimeEnvironment::new();
        assert!(env.variables().is_empty());
    }

    #[test]
    fn test_environment_add_variable() {
        let mut env = RuntimeEnvironment::new();
        env.add_variable("FOO", "bar");
        assert_eq!(env.variables().len(), 1);
        assert_eq!(env.variables()[0], ("FOO".to_string(), "bar".to_string()));
    }

    #[test]
    fn test_runtime_context_preserves_identity() {
        let temp = TempDir::new().unwrap();
        create_installed_package(temp.path());

        let identity = "testdev.test";
        let ctx = prepare(
            identity,
            Some(temp.path()),
            Some(&temp.path().join("runtims")),
        )
        .unwrap();
        assert_eq!(ctx.identity, identity);
        assert_eq!(ctx.manifest.name, "test");
        assert_eq!(ctx.manifest.developer, "testdev");
    }

    #[test]
    fn test_prepare_never_executes() {
        let temp = TempDir::new().unwrap();
        create_installed_package(temp.path());

        let identity = "testdev.test";
        let ctx = prepare(
            identity,
            Some(temp.path()),
            Some(&temp.path().join("runtims")),
        );
        assert!(ctx.is_ok());
        let ctx = ctx.unwrap();
        assert_eq!(ctx.state, RuntimeState::Prepared);
    }
}
