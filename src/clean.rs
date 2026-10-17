use std::path::PathBuf;

pub struct CleanTarget {
    pub name: &'static str,
    pub path: PathBuf,
    pub needs_sudo: bool,
    /// Command hint shown when sudo is required
    pub sudo_cmd: Option<&'static str>,
}

pub fn targets() -> Vec<CleanTarget> {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/".into());
    let h = |p: &str| PathBuf::from(&home).join(p);
    vec![
        CleanTarget { name: "AUR/yay build cache", path: h(".cache/yay"), needs_sudo: false, sudo_cmd: None },
        CleanTarget { name: "Thumbnails", path: h(".cache/thumbnails"), needs_sudo: false, sudo_cmd: None },
        CleanTarget { name: "Fontconfig cache", path: h(".cache/fontconfig"), needs_sudo: false, sudo_cmd: None },
        CleanTarget { name: "Mesa shader cache", path: h(".cache/mesa_shader_cache"), needs_sudo: false, sudo_cmd: None },
        CleanTarget { name: "Nvim logs", path: h(".local/state/nvim"), needs_sudo: false, sudo_cmd: None },
        CleanTarget { name: "Flatpak app caches", path: h(".var/app"), needs_sudo: false, sudo_cmd: None },
        CleanTarget { name: "Pacman package cache", path: PathBuf::from("/var/cache/pacman/pkg"), needs_sudo: true, sudo_cmd: Some("sudo paccache -rk2") },
        CleanTarget { name: "Systemd journal", path: PathBuf::from("/var/log/journal"), needs_sudo: true, sudo_cmd: Some("sudo journalctl --vacuum-size=50M") },
        CleanTarget { name: "Old temp files", path: PathBuf::from("/tmp"), needs_sudo: true, sudo_cmd: Some("sudo systemd-tmpfiles --clean") },
    ]
}

/// Total size of a directory in bytes (best effort, depth-limited, no symlink following)
pub fn dir_size(path: &std::path::Path) -> u64 {
    dir_size_depth(path, 6)
}

fn dir_size_depth(path: &std::path::Path, depth: u32) -> u64 {
    if depth == 0 {
        return 0;
    }
    let meta = match std::fs::symlink_metadata(path) {
        Ok(m) => m,
        Err(_) => return 0,
    };
    if meta.file_type().is_symlink() {
        return 0;
    }
    if meta.is_file() {
        return meta.len();
    }
    let mut sum = 0u64;
    if let Ok(rd) = std::fs::read_dir(path) {
        for entry in rd.flatten() {
            let p = entry.path();
            if p.is_dir() {
                sum += dir_size_depth(&p, depth - 1);
            } else if let Ok(m) = entry.metadata() {
                sum += m.len();
            }
        }
    }
    sum
}

pub fn flatpak_cache_size() -> u64 {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/".into());
    let base = PathBuf::from(home).join(".var/app");
    let mut sum = 0;
    if let Ok(rd) = std::fs::read_dir(&base) {
        for e in rd.flatten() {
            let c = e.path().join("cache");
            if c.is_dir() {
                sum += dir_size(&c);
            }
        }
    }
    sum
}

/// Remove a user cache dir (recreated on demand)
pub fn delete(path: &std::path::Path) -> std::io::Result<()> {
    if path.is_dir() {
        std::fs::remove_dir_all(path)
    } else {
        std::fs::remove_file(path)
    }
}

pub fn delete_flatpak_caches() {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/".into());
    let base = PathBuf::from(home).join(".var/app");
    if let Ok(rd) = std::fs::read_dir(&base) {
        for e in rd.flatten() {
            let c = e.path().join("cache");
            if c.is_dir() {
                let _ = std::fs::remove_dir_all(&c);
            }
        }
    }
}

pub fn human(bytes: u64) -> String {
    const U: [&str; 5] = ["B", "K", "M", "G", "T"];
    let mut b = bytes as f64;
    let mut i = 0;
    while b >= 1024.0 && i < U.len() - 1 {
        b /= 1024.0;
        i += 1;
    }
    format!("{b:.1} {}", U[i])
}
