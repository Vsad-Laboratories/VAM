//! VAM package model and manifest validation.
//!
//! The `package` module is the package-domain boundary of VAM Core. It owns
//! the in-memory representation of a VAM Package and its manifest, plus the
//! validation that every manifest must satisfy before VAM reasons about a
//! package.
//!
//! This goal implements **only** the data model and validation. It does not
//! read or parse manifest files:
//!
//! - The on-disk manifest format is TOML (see `Package.md`, OD-02). TOML
//!   parsing is deferred; the future `.vampkg`/repository goals will parse
//!   the archive and construct a `Manifest` in memory.
//! - No `.vampkg` handling, installation, extraction, execution, runtime,
//!   repository, dependency, signature, capability, or privilege logic exists
//!   here. Those subsystems are out of scope and remain elsewhere.
//!
//! # Package identity
//!
//! A package's machine-readable identity is the `<developer>.<name>` composite
//! key (see `Package.md`, OD-03). It is represented by [`PackageId`]. The
//! `version` and `release` fields identify a specific build of a package but
//! are not part of the composite identity.
//!
//! # Validation and errors
//!
//! [`Manifest::validate`] checks that every required field is present and
//! non-empty and that the package identity is valid. Validation failures are
//! expected, correctable input problems (the manifest is input data), so they
//! are reported as `ErrorKind::Usage` through the existing error model. No
//! secrets, credentials, or filesystem paths are exposed in validation
//! messages.
//!
//! # Fields
//!
//! Required fields mirror `Package.md` §4.1, minus the out-of-scope
//! `dependencies`, `capabilities`, and `integrity` fields. `configuration`
//! is optional per the specification and is therefore not modeled here.
//!
//! | Field | Required | Notes |
//! |-------|----------|-------|
//! | `name` | Yes | package name (identity) |
//! | `developer` | Yes | developer identity (identity) |
//! | `version` | Yes | build version |
//! | `release` | Yes | build release identifier |
//! | `description` | Yes | brief human description |
//! | `purpose` | Yes | package purpose/category |
//! | `package_type` | Yes | package type classification |
//!
//! Value semantics for `purpose` and `package_type` (e.g. allowed enums) are
//! unspecified by the current specification and are intentionally deferred;
//! validation enforces presence only.

use crate::error::{Error, Result};
use std::fmt;

/// A package's unique `<developer>.<name>` composite identity.
///
/// Two packages from different developers with the same name are distinct.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageId {
    developer: String,
    name: String,
}

impl PackageId {
    pub fn new(developer: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            developer: developer.into(),
            name: name.into(),
        }
    }

    pub fn developer(&self) -> &str {
        &self.developer
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    /// Validates the identity: both `developer` and `name` must be non-empty.
    pub fn validate(&self) -> Result<()> {
        if self.developer.trim().is_empty() {
            return Err(Error::usage("manifest field 'developer' must not be empty"));
        }
        if self.name.trim().is_empty() {
            return Err(Error::usage("manifest field 'name' must not be empty"));
        }
        Ok(())
    }
}

impl fmt::Display for PackageId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.developer, self.name)
    }
}

/// In-memory model of a VAM package manifest.
///
/// Construct a `Manifest` and call [`Manifest::validate`] to confirm it
/// satisfies the required-field contract before use. The manifest is plain,
/// owned data; persistence/parsing is handled by the archive/repository
/// layers, neither of which exists at this foundation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Manifest {
    pub name: String,
    pub developer: String,
    pub version: String,
    pub release: String,
    pub description: String,
    pub purpose: String,
    pub package_type: String,
}

impl Manifest {
    /// The package's composite identity.
    pub fn package_id(&self) -> PackageId {
        PackageId::new(self.developer.as_str(), self.name.as_str())
    }

    /// Validates the manifest against the required-field contract.
    ///
    /// Returns `ErrorKind::Usage` for any missing or empty required field.
    /// `version`, `release`, `purpose`, and `package_type` are checked for
    /// presence only; their value semantics are deferred per the specification.
    pub fn validate(&self) -> Result<()> {
        self.package_id().validate()?;

        for (field, value) in [
            ("version", &self.version),
            ("release", &self.release),
            ("description", &self.description),
            ("purpose", &self.purpose),
            ("package_type", &self.package_type),
        ] {
            if value.trim().is_empty() {
                return Err(Error::usage(format!(
                    "manifest field '{field}' must not be empty"
                )));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::ErrorKind;

    fn valid_manifest() -> Manifest {
        Manifest {
            name: "sudo-cleaner".to_string(),
            developer: "vsad".to_string(),
            version: "1.0.2".to_string(),
            release: "1".to_string(),
            description: "Removes stale sudo timestamp caches".to_string(),
            purpose: "cleanup".to_string(),
            package_type: "standard".to_string(),
        }
    }

    #[test]
    fn test_package_id_display_is_composite_key() {
        let id = PackageId::new("vsad", "sudo-cleaner");
        assert_eq!(id.to_string(), "vsad.sudo-cleaner");
    }

    #[test]
    fn test_package_id_validate_succeeds() {
        let id = PackageId::new("vsad", "sudo-cleaner");
        assert!(id.validate().is_ok());
    }

    #[test]
    fn test_package_id_validate_rejects_empty_developer() {
        let id = PackageId::new("", "sudo-cleaner");
        let err = id.validate().unwrap_err();
        assert_eq!(err.kind(), ErrorKind::Usage);
        assert!(err.message().contains("developer"));
    }

    #[test]
    fn test_package_id_validate_rejects_empty_name() {
        let id = PackageId::new("vsad", "");
        let err = id.validate().unwrap_err();
        assert_eq!(err.kind(), ErrorKind::Usage);
        assert!(err.message().contains("name"));
    }

    #[test]
    fn test_package_id_validate_rejects_whitespace_only() {
        let id = PackageId::new("  ", "sudo-cleaner");
        assert!(id.validate().is_err());
    }

    #[test]
    fn test_manifest_validate_succeeds_for_valid_manifest() {
        let manifest = valid_manifest();
        assert!(manifest.validate().is_ok());
    }

    #[test]
    fn test_manifest_package_id_uses_developer_and_name() {
        let manifest = valid_manifest();
        assert_eq!(manifest.package_id().to_string(), "vsad.sudo-cleaner");
    }

    #[test]
    fn test_manifest_validate_rejects_empty_name() {
        let mut manifest = valid_manifest();
        manifest.name = String::new();
        let err = manifest.validate().unwrap_err();
        assert_eq!(err.kind(), ErrorKind::Usage);
        assert!(err.message().contains("name"));
    }

    #[test]
    fn test_manifest_validate_rejects_empty_developer() {
        let mut manifest = valid_manifest();
        manifest.developer = String::new();
        let err = manifest.validate().unwrap_err();
        assert_eq!(err.kind(), ErrorKind::Usage);
        assert!(err.message().contains("developer"));
    }

    #[test]
    fn test_manifest_validate_rejects_empty_version() {
        let mut manifest = valid_manifest();
        manifest.version = String::new();
        let err = manifest.validate().unwrap_err();
        assert_eq!(err.kind(), ErrorKind::Usage);
        assert!(err.message().contains("version"));
    }

    #[test]
    fn test_manifest_validate_rejects_empty_description() {
        let mut manifest = valid_manifest();
        manifest.description = String::new();
        let err = manifest.validate().unwrap_err();
        assert_eq!(err.kind(), ErrorKind::Usage);
        assert!(err.message().contains("description"));
    }

    #[test]
    fn test_manifest_validate_rejects_whitespace_only_release() {
        let mut manifest = valid_manifest();
        manifest.release = "   ".to_string();
        assert!(manifest.validate().is_err());
    }

    #[test]
    fn test_manifest_validate_rejects_empty_package_type() {
        let mut manifest = valid_manifest();
        manifest.package_type = String::new();
        let err = manifest.validate().unwrap_err();
        assert_eq!(err.kind(), ErrorKind::Usage);
        assert!(err.message().contains("package_type"));
    }

    #[test]
    fn test_manifest_validate_reports_first_required_failure() {
        // With multiple invalid identity fields, the first checked (developer)
        // is reported, consistent with the identity model taking precedence.
        let mut manifest = valid_manifest();
        manifest.developer = String::new();
        manifest.name = String::new();
        let err = manifest.validate().unwrap_err();
        assert_eq!(err.kind(), ErrorKind::Usage);
        assert!(err.message().contains("developer"));
    }

    #[test]
    fn test_validation_error_messages_are_fixed_templates() {
        // Error messages name only the offending field via a fixed template,
        // so field values (which may contain sensitive strings) are never
        // echoed in diagnostics or user-facing output.
        let mut manifest = valid_manifest();
        manifest.description = String::new(); // triggers the error
        manifest.version = "/etc/secret.conf".to_string(); // unrelated field
        let err = manifest.validate().unwrap_err();
        assert_eq!(
            err.message(),
            "manifest field 'description' must not be empty"
        );
        assert!(!err.message().contains("/etc/secret.conf"));
    }
}
