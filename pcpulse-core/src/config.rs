//! TOML config loading/writing.
//!
//! User config lives in the platform config directory (`pcpulse`). Missing
//! files are materialized from the commented default TOML template so the user
//! always has an editable threshold file.

use anyhow::Result;
use directories::BaseDirs;
use std::fs::create_dir_all;
use std::fs::read_to_string;
use std::fs::write;
use std::path::PathBuf;

#[derive(serde::Deserialize)]
pub struct Config {
    #[serde(default)]
    pub cpu_temp_warn_c: f64,
    #[serde(default)]
    pub cpu_temp_bad_c: f64,
    #[serde(default)]
    pub cpu_usage_warn_pct: f64,
    #[serde(default)]
    pub cpu_usage_bad_pct: f64,
    #[serde(default)]
    pub ram_used_warn_pct: f64,
    #[serde(default)]
    pub ram_used_bad_pct: f64,
    #[serde(default)]
    pub disk_free_warn_gb: f64,
    #[serde(default)]
    pub disk_free_bad_gb: f64,
    #[serde(default)]
    pub net_error_warn: u64,
    #[serde(default)]
    pub top_processes: u32,
    #[serde(default)]
    pub show_per_core: bool,
}

pub fn default_config() -> Config {
    Config {
        cpu_temp_warn_c: 70.0,
        cpu_temp_bad_c: 95.0,
        cpu_usage_warn_pct: 75.0,
        cpu_usage_bad_pct: 90.0,
        ram_used_warn_pct: 70.0,
        ram_used_bad_pct: 90.0,
        disk_free_warn_gb: 50.0,
        disk_free_bad_gb: 20.0,
        net_error_warn: 0_u64,
        top_processes: 8_u32,
        show_per_core: true,
    }
}

pub fn config_path() -> PathBuf {
    match BaseDirs::new() {
        Some(base) => base.config_dir().join("pcpulse").join("config.toml").to_path_buf(),
        None => PathBuf::from("pcpulse").join("config.toml"),
    }
}

pub fn write_default_config() -> PathBuf {
    let file = config_path();

    if !file.exists() {
        if let Some(parent) = file.parent() {
            let _ = create_dir_all(parent);
        }
        let default: String = include_str!("../default.toml").to_string();
        let _ = write(&file, default.as_bytes());
    }

    file
}

pub fn load_config() -> Result<Config> {
    let file = write_default_config();

    match read_to_string(file) {
        Ok(text) => {
            match toml::from_str(&text) {
                Ok(cfg) => Ok(cfg),
                Err(_) => Ok(default_config()),
            }
        }
        Err(_) => Ok(default_config()),
    }
}
