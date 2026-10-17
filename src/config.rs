use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Default, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Automation {
    pub prune_yay_cache: bool,
    pub prune_thumbnails: bool,
    pub prune_mesa_shader: bool,
    pub prune_flatpak_caches: bool,
    pub low_disk_watch: bool,
    pub low_disk_threshold_pct: u8,
    pub service_watchdog: bool,
    pub watch_services: Vec<String>,
    pub high_cpu_alert: bool,
    pub cpu_alert_threshold: f32,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub automation: Automation,
}

impl Config {
    pub fn path() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/".into());
        PathBuf::from(home).join(".config/vam/config.toml")
    }

    /// Returns the config plus an optional warning.
    ///
    /// A file that exists but cannot be parsed yields defaults *and* a warning:
    /// the UI has to say so, and `save()` must refuse to clobber it — silently
    /// replacing unparseable settings with defaults loses the user's config.
    pub fn load() -> (Self, Option<String>) {
        let p = Self::path();
        let mut warn = None;
        let mut cfg: Self = match std::fs::read_to_string(&p) {
            Ok(s) => match toml::from_str(&s) {
                Ok(c) => c,
                Err(e) => {
                    warn = Some(format!("{}: {e}", p.display()));
                    Self::default()
                }
            },
            Err(_) => Self::default(),
        };
        if cfg.automation.low_disk_threshold_pct == 0 {
            cfg.automation.low_disk_threshold_pct = 90;
        }
        if cfg.automation.cpu_alert_threshold == 0.0 {
            cfg.automation.cpu_alert_threshold = 80.0;
        }
        (cfg, warn)
    }

    /// Returns false when the write was refused because the existing file could not
/// be parsed. Callers must report that instead of claiming success.
pub fn save(&self) -> bool {
        let p = Self::path();
        // Guard: never overwrite a config we were unable to parse.
        if let Ok(s) = std::fs::read_to_string(&p) {
            if toml::from_str::<Self>(&s).is_err() {
                return false;
            }
        }
        if let Some(parent) = p.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        match toml::to_string_pretty(self) {
            Ok(s) => std::fs::write(&p, s).is_ok(),
            Err(_) => false,
        }
    }
}
