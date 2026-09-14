use crate::error::{Error, ErrorKind, Result};
use crate::package::Manifest;
use std::collections::HashSet;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use tar::Header;
use zstd;

/// Magic number for VAM package files: "VAMP"
pub const VAM_MAGIC: [u8; 4] = *b"VAMP";
/// Version of the container format
pub const CONTAINER_VERSION: u32 = 1;

/// Information about a package read from its manifest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageInfo {
    pub name: String,
    pub developer: String,
    pub version: String,
    pub release: String,
    pub description: String,
    pub purpose: String,
    pub package_type: String,
}

/// Serializes a PackageInfo to a TOML string (simple key-value).
fn serialize_toml(info: &PackageInfo) -> String {
    fn escape_toml_string(s: &str) -> String {
        let mut escaped = String::with_capacity(s.len() + 2);
        escaped.push('"');
        for c in s.chars() {
            match c {
                '"' => escaped.push_str(r#"\""#),
                '\\' => escaped.push_str(r#"\\"#),
                _ => escaped.push(c),
            }
        }
        escaped.push('"');
        escaped
    }

    let mut lines = Vec::new();
    macro_rules! add_field {
        ($key:expr, $value:expr) => {
            lines.push(format!(
                "{} = {}",
                $key,
                escape_toml_string($value)
            ));
        };
    }
    add_field!("name", &info.name);
    add_field!("developer", &info.developer);
    add_field!("version", &info.version);
    add_field!("release", &info.release);
    add_field!("description", &info.description);
    add_field!("purpose", &info.purpose);
    add_field!("package_type", &info.package_type);
    lines.join("\n")
}
/// Expects exactly the seven keys as strings.
fn deserialize_toml(toml: &str) -> Result<PackageInfo> {
    let mut name = None;
    let mut developer = None;
    let mut version = None;
    let mut release = None;
    let mut description = None;
    let mut purpose = None;
    let mut package_type = None;

    for line in toml.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some(eq_idx) = line.find('=') else {
            return Err(Error::usage("invalid TOML line"));
        };
        let key = line[..eq_idx].trim();
        let value = line[eq_idx + 1..].trim();
        if !value.starts_with('"') || !value.ends_with('"') {
            return Err(Error::usage("expected string value"));
        }
        let inner = &value[1..value.len() - 1];
        match key {
            "name" => name = Some(inner.to_string()),
            "developer" => developer = Some(inner.to_string()),
            "version" => version = Some(inner.to_string()),
            "release" => release = Some(inner.to_string()),
            "description" => description = Some(inner.to_string()),
            "purpose" => purpose = Some(inner.to_string()),
            "package_type" => package_type = Some(inner.to_string()),
            _ => return Err(Error::usage("unknown key in manifest")),
        }
    }

    let info = PackageInfo {
        name: name.ok_or_else(|| Error::usage("missing name field"))?,
        developer: developer.ok_or_else(|| Error::usage("missing developer field"))?,
        version: version.ok_or_else(|| Error::usage("missing version field"))?,
        release: release.ok_or_else(|| Error::usage("missing release field"))?,
        description: description.ok_or_else(|| Error::usage("missing description field"))?,
        purpose: purpose.ok_or_else(|| Error::usage("missing purpose field"))?,
        package_type: package_type.ok_or_else(|| Error::usage("missing package_type field"))?,
    };
    Ok(info)
}

/// Validates a package path according to security rules.
/// Returns ErrorKind::Usage on violation.
pub fn validate_package_path<P: AsRef<Path>>(path: P) -> Result<()> {
    let path = path.as_ref();
    // 1. No absolute paths
    if path.is_absolute() {
        return Err(Error::usage("package path must not be absolute"));
    }
    // 2. No `..` components
    for component in path.components() {
        if matches!(component, std::path::Component::ParentDir) {
            return Err(Error::usage("package path must not contain `..`"));
        }
    }
    Ok(())
}

/// Returns the relative path from base to path, assuming path is under base.
/// Returns an owned String with forward slashes.
fn relative_path(base: &Path, path: &Path) -> Result<String> {
    let mut base_iter = base.components();
    let mut path_iter = path.components();
    loop {
        match (base_iter.next(), path_iter.next()) {
            (None, None) => return Ok(String::new()),
            (None, Some(_comp)) => {
                // base is a prefix of path
                let mut rel = PathBuf::new();
                for c in path_iter {
                    rel = rel.join(c);
                }
                let mut s = rel
                    .to_str()
                    .ok_or_else(|| Error::usage("path contains invalid Unicode"))?
                    .to_string();
                // Convert backslashes to forward slashes for consistency
                s = s.replace('\\', "/");
                return Ok(s);
            }
            (Some(b), Some(p)) if b == p => continue,
            _ => return Err(Error::usage("path is not under base")),
        }
    }
}

/// Collects files from a payload directory recursively, returning a sorted list of
/// (relative_path, absolute_path) pairs. The relative paths are normalized to use
/// forward slash and are relative to the payload root.
fn collect_files<P: AsRef<Path>>(payload_root: P) -> Result<Vec<(String, PathBuf)>> {
    let payload_root = payload_root.as_ref();
    if !payload_root.is_dir() {
        return Err(Error::usage("payload directory does not exist"));
    }

    let mut files = Vec::new();
    let mut seen = HashSet::new();

    for entry in fs::read_dir(payload_root)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let rel_str = relative_path(payload_root, &path)?;
            // Validate the relative path (no absolute, no ..)
            validate_package_path(&rel_str)?;
            // Check for duplicate logical paths
            if !seen.insert(rel_str.clone()) {
                return Err(Error::usage("duplicate logical path in payload"));
            }
            files.push((rel_str, path));
        } else if path.is_dir() {
            // Recurse into subdirectory
            let mut subfiles = collect_files(&path)?;
            // Adjust relative paths to include the directory name
            let prefix = relative_path(payload_root, &path)?;
            for (rel, abs) in subfiles.drain(..) {
                let mut new_rel = String::new();
                if !prefix.is_empty() {
                    new_rel.push_str(&prefix);
                    new_rel.push('/');
                }
                new_rel.push_str(&rel);
                // Validate the new relative path
                validate_package_path(&new_rel)?;
                if !seen.insert(new_rel.clone()) {
                    return Err(Error::usage("duplicate logical path in payload"));
                }
                files.push((new_rel, abs));
            }
        }
    }

    // Sort by relative path for determinism
    files.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(files)
}

/// Creates a .vampkg file from a source directory containing manifest.toml, entrypoint, and payload/.
///
/// # Arguments
///
/// * `source_dir` - Directory containing the package source (must have manifest.toml, entrypoint file, and payload/ subdirectory)
/// * `output` - Path to write the .vampkg file
pub fn create<P: AsRef<Path>, Q: AsRef<Path>>(source_dir: P, output: Q) -> Result<()> {
    let source_dir = source_dir.as_ref();
    let output = output.as_ref();

    // Read manifest.toml
    let manifest_path = source_dir.join("manifest.toml");
    let manifest_toml = fs::read_to_string(&manifest_path)?;
    let info = deserialize_toml(&manifest_toml)?;

    // Validate manifest fields using the Manifest model
    let manifest = Manifest {
        name: info.name.clone(),
        developer: info.developer.clone(),
        version: info.version.clone(),
        release: info.release.clone(),
        description: info.description.clone(),
        purpose: info.purpose.clone(),
        package_type: info.package_type.clone(),
    };
    manifest.validate()?;

    // Read entrypoint file (assumed to be named "entrypoint" at the root of the source directory)
    let entrypoint_path = source_dir.join("entrypoint");
    if !entrypoint_path.is_file() {
        return Err(Error::usage("entrypoint file not found"));
    }
    let entrypoint_data = fs::read(&entrypoint_path)?;
    let entrypoint_rel = "entrypoint";
    validate_package_path(entrypoint_rel)?;

    // Collect files from payload/ directory
    let payload_dir = source_dir.join("payload");
    let payload_files = collect_files(&payload_dir)?;

    // Build tar archive in memory
    let mut tar_builder = tar::Builder::new(vec![]);
    // Add manifest.toml
    {
        let mut header = Header::new_gnu();
        header.set_size(manifest_toml.len() as u64);
        header.set_mode(0o644);
        header.set_mtime(0);
        tar_builder.append_data(&mut header, "manifest.toml", manifest_toml.as_bytes())?;
    }
    // Add entrypoint
    {
        let mut header = Header::new_gnu();
        header.set_size(entrypoint_data.len() as u64);
        header.set_mode(0o644);
        header.set_mtime(0);
        tar_builder.append_data(&mut header, entrypoint_rel, &*entrypoint_data)?;
    }
    // Add payload files
    for (rel_path, abs_path) in &payload_files {
        let data = fs::read(abs_path)?;
        let mut header = Header::new_gnu();
        header.set_size(data.len() as u64);
        header.set_mode(0o644);
        header.set_mtime(0);
        tar_builder.append_data(&mut header, rel_path, &*data)?;
    }
    let tar_data = tar_builder.into_inner()?;

    // Compress with zstd
    let compressed = zstd::stream::encode_all(&*tar_data, 0)?;

    // Write container header: magic + version (u32, little endian) + compressed data
    let mut output_file = fs::File::create(output)?;
    output_file.write_all(&VAM_MAGIC)?;
    output_file.write_all(&CONTAINER_VERSION.to_le_bytes())?;
    output_file.write_all(&*compressed)?;

    Ok(())
}

/// Inspects a .vampkg file, returning the PackageInfo if valid.
///
/// # Arguments
///
/// * `path` - Path to the .vampkg file
pub fn inspect<P: AsRef<Path>>(path: P) -> Result<PackageInfo> {
    let path = path.as_ref();
    let mut file = fs::File::open(path)?;
    // Read header: 4 bytes magic + 4 bytes version
    let mut magic = [0u8; 4];
    file.read_exact(&mut magic)?;
    if magic != VAM_MAGIC {
        return Err(Error::usage("invalid VAM package magic"));
    }
    let mut version_bytes = [0u8; 4];
    file.read_exact(&mut version_bytes)?;
    let version = u32::from_le_bytes(version_bytes);
    if version != CONTAINER_VERSION {
        return Err(Error::usage("unsupported container version"));
    }
    // The rest is the compressed tar data
    let mut compressed = Vec::new();
    file.read_to_end(&mut compressed)?;

    // Decompress
    let tar_data = zstd::stream::decode_all(&*compressed)?;

    // Parse tar archive
    let mut manifest_toml = None;
    let mut entrypoint_data = None;
    let mut payload_files = Vec::new();

    let mut archive = tar::Archive::new(&*tar_data);
    for entry in archive.entries()? {
        let mut entry = entry?;
        let rel_path = entry.path()?
            .to_str()
            .ok_or_else(|| Error::usage("path contains invalid Unicode"))?
            .to_string();
        // Validate the path inside the archive (should be safe, but we check)
        validate_package_path(&rel_path)?;
        let mut contents = Vec::new();
        entry.read_to_end(&mut contents)?;
        match rel_path.as_str() {
            "manifest.toml" => {
                if manifest_toml.is_some() {
                    return Err(Error::usage("duplicate manifest.toml in archive"));
                }
                manifest_toml = Some(String::from_utf8(contents)
                    .map_err(|e| Error::with_source(ErrorKind::Internal, "manifest.toml is not valid UTF-8", e))?);
            }
            "entrypoint" => {
                if entrypoint_data.is_some() {
                    return Err(Error::usage("duplicate entrypoint in archive"));
                }
                entrypoint_data = Some(contents);
            }
            _ => {
                // Assume it's a payload file
                payload_files.push((rel_path, contents));
            }
        }
    }

    let manifest_toml = manifest_toml.ok_or_else(|| Error::usage("manifest.toml not found in archive"))?;
    let info = deserialize_toml(&manifest_toml)?;

    // Validate the manifest
    let manifest = Manifest {
        name: info.name.clone(),
        developer: info.developer.clone(),
        version: info.version.clone(),
        release: info.release.clone(),
        description: info.description.clone(),
        purpose: info.purpose.clone(),
        package_type: info.package_type.clone(),
    };
    manifest.validate()?;

    // Validate entrypoint exists
    let _ = entrypoint_data.ok_or_else(|| Error::usage("entrypoint not found in archive"))?;

    // Validate payload files: check for duplicates (should not happen if we built correctly, but we check)
    let mut seen = HashSet::new();
    for (rel_path, _) in &payload_files {
        if !seen.insert(rel_path) {
            return Err(Error::usage("duplicate logical path in payload"));
        }
        validate_package_path(rel_path)?;
    }

    Ok(info)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
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
    fn test_create_and_inspect() {
        let temp = TempDir::new().unwrap();
        let src_dir = temp.path().join("src");
        fs::create_dir(&src_dir).unwrap();
        create_sample_package(&src_dir);

        let out_path = temp.path().join("out.vampkg");
        assert!(create(&src_dir, &out_path).is_ok());

        let info = inspect(&out_path).unwrap();
        assert_eq!(info.name, "test");
        assert_eq!(info.developer, "testdev");
        assert_eq!(info.version, "1.0");
        assert_eq!(info.release, "1");
        assert_eq!(info.description, "A test package");
        assert_eq!(info.purpose, "test");
        assert_eq!(info.package_type, "standard");
    }

    #[test]
    fn test_manifest_roundtrip() {
        let info = PackageInfo {
            name: "hello".to_string(),
            developer: "dev".to_string(),
            version: "2.0".to_string(),
            release: "2".to_string(),
            description: "desc".to_string(),
            purpose: "purp".to_string(),
            package_type: "type".to_string(),
        };
        let toml = serialize_toml(&info);
        let parsed = deserialize_toml(&toml).unwrap();
        assert_eq!(parsed, info);
    }

    #[test]
    fn test_validate_package_path_rejects_absolute() {
        assert!(validate_package_path("/tmp").is_err());
    }

    #[test]
    fn test_validate_package_path_rejects_parent_dir() {
        assert!(validate_package_path("../foo").is_err());
    }

    #[test]
    fn test_collect_files_deterministic_order() {
        let temp = TempDir::new().unwrap();
        let payload_dir = temp.path().join("payload");
        fs::create_dir(&payload_dir).unwrap();
        // Create files in reverse alphabetical order
        fs::write(payload_dir.join("z.txt"), "z").unwrap();
        fs::write(payload_dir.join("a.txt"), "a").unwrap();
        fs::create_dir(payload_dir.join("b")).unwrap();
        fs::write(payload_dir.join("b").join("m.txt"), "m").unwrap();
        fs::write(payload_dir.join("b").join("a.txt"), "a").unwrap();

        let files = collect_files(&payload_dir).unwrap();
        // Expect sorted by relative path: a.txt, b/a.txt, b/m.txt, z.txt
        assert_eq!(files.len(), 4);
        assert_eq!(files[0].0, "a.txt");
        assert_eq!(files[1].0, "b/a.txt");
        assert_eq!(files[2].0, "b/m.txt");
        assert_eq!(files[3].0, "z.txt");
    }

    #[test]
    fn test_inspect_rejects_bad_magic() {
        let temp = TempDir::new().unwrap();
        let bad = temp.path().join("bad.vampkg");
        fs::write(bad.clone(), "BADM").unwrap();
        assert!(inspect(&bad).is_err());
    }

    #[test]
    fn test_inspect_rejects_bad_version() {
        let temp = TempDir::new().unwrap();
        let bad = temp.path().join("bad.vampkg");
        let mut buf = Vec::new();
        buf.extend_from_slice(b"VAMP");
        buf.extend_from_slice(&2u32.to_le_bytes()); // version 2
        fs::write(bad.clone(),         fs::write(bad.clone(),         fs::write(bad.clone(), fs::write(bad, &buf)buf).unwrap();buf).unwrap();buf).unwrap();
        assert!(inspect(&bad).is_err());
    }
}