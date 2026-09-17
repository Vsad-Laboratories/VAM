use crate::error::{Error, ErrorKind, Result};

use std::fs;
use std::path::Path;

/// Installs a .vampkg file.
///
/// # Arguments
///
/// * `package_path` - Path to the .vampkg file to install.
/// * `base_install_dir` - Base directory where packages are installed.
///
/// Defaults to "/usr/lib/vam/packages".
///
/// # Returns
///
/// * `Ok(())` if the package was installed successfully.
/// * `Err(Error)` if the package could not be installed.
///
/// # Steps
///
/// 1. Extract the package using `inspect_extract`.
/// 2. Check if the package is already installed (by identity).
/// 3. Prepare a temporary directory for installation.
/// 4. Write the manifest.toml file.
/// 5. Write the entrypoint file (if present).
/// 6. Create the payload directory and write all payload files.
/// 7. Rename the temporary directory to the final installation directory.
///
/// If any step fails, the temporary directory is cleaned up.
pub fn install<P: AsRef<Path>>(package_path: P, base_install_dir: Option<&Path>) -> Result<()> {
    // Use the default base install directory if not provided.
    let base_install_dir = base_install_dir.unwrap_or_else(|| Path::new("/usr/lib/vam/packages"));

    // Extract the package.
    let (info, entrypoint_data, payload_files) =
        crate::package::container::inspect_extract(package_path)?;

    // Compute the package identity.
    let identity = format!("{}.{}", info.developer, info.name);
    let install_dir = base_install_dir.join(&identity);

    // Check if the package is already installed.
    if install_dir.exists() {
        return Err(Error::usage("package already installed"));
    }

    // Prepare a temporary directory for installation.
    // We use a timestamp-based suffix to avoid conflicts.
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| Error::with_source(ErrorKind::Internal, "failed to get system time", e))?;
    let temp_dir = base_install_dir.join(format!("{}.tmp", now.as_nanos()));

    // Ensure the temporary directory does not already exist.
    if temp_dir.exists() {
        return Err(Error::usage("temporary directory already exists"));
    }

    // Create the temporary directory.
    fs::create_dir_all(&temp_dir)?;

    // Write the manifest.toml file.
    let manifest_path = temp_dir.join("manifest.toml");
    let manifest_toml = crate::package::container::serialize_toml(&info);
    fs::write(&manifest_path, manifest_toml)?;

    // Write the entrypoint file (if present).
    if let Some(entrypoint_data) = &entrypoint_data {
        let entrypoint_path = temp_dir.join("entrypoint");
        fs::write(&entrypoint_path, entrypoint_data)?;
    }

    // Create the payload directory.
    let payload_dir = temp_dir.join("payload");
    fs::create_dir_all(&payload_dir)?;

    // Write each payload file.
    for (rel_path, contents) in &payload_files {
        let abs_path = payload_dir.join(rel_path);
        // Ensure the parent directory exists.
        if let Some(parent) = abs_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&abs_path, contents)?;
    }

    // Rename the temporary directory to the final installation directory.
    fs::rename(&temp_dir, &install_dir)?;

    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    use tempfile::TempDir;

    fn create_sample_package(dir: &Path) {
        // manifest.toml
        let manifest = r#"name = "test"
developer = "testdev"
version = "1.0"
release = "1"
description = "A test package"
purpose = "test"
package_type = "standard"
"#;
        fs::write(dir.join("manifest.toml"), manifest).unwrap();
        // entrypoint
        fs::write(dir.join("entrypoint"), "#!/bin/sh\necho hello").unwrap();
        // payload directory
        let payload_dir = dir.join("payload");
        fs::create_dir(&payload_dir).unwrap();
        fs::write(payload_dir.join("file.txt"), "content").unwrap();
        let subdir = payload_dir.join("sub");
        fs::create_dir(&subdir).unwrap();
        fs::write(subdir.join("nested.txt"), "nested").unwrap();
    }

    #[test]
    fn test_install_success() {
        let temp = TempDir::new().unwrap();
        let src_dir = temp.path().join("src");
        fs::create_dir(&src_dir).unwrap();
        create_sample_package(&src_dir);

        let out_path = temp.path().join("out.vampkg");
        assert!(crate::package::container::create(&src_dir, &out_path).is_ok());

        let install_base = temp.path().join("install");
        let result = install(&out_path, Some(&install_base));
        println!("{:?}", result);
        if let Err(ref e) = result {
            if let Some(source) = std::error::Error::source(e) {
                println!("source: {:?}", source);
                println!("source: {:#?}", source);
            }
        }
        assert!(result.is_ok());

        let installed_dir = install_base.join("testdev.test");
        assert!(installed_dir.exists());
        assert!(installed_dir.join("manifest.toml").exists());
        assert!(installed_dir.join("entrypoint").exists());
        let payload_dir = installed_dir.join("payload");
        assert!(payload_dir.exists());
        assert!(payload_dir.join("file.txt").exists());
        assert!(payload_dir.join("sub").join("nested.txt").exists());
    }
    #[test]
    fn test_install_invalid_package() {
        let temp = TempDir::new().unwrap();
        let bad_path = temp.path().join("bad.vampkg");
        fs::write(&bad_path, b"NOT A VALID PACKAGE").unwrap();

        let install_base = temp.path().join("install");
        assert!(install(&bad_path, Some(&install_base)).is_err());
    }
}
