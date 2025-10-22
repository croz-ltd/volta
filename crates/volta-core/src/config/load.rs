use super::Settings;
use crate::error::{ErrorKind, VoltaError};
use std::fs;
use std::path::PathBuf;
use toml;

// Platform-aware config path
fn config_file_path() -> Option<PathBuf> {
    // Honor VOLTA_HOME if set
    if let Ok(home) = std::env::var("VOLTA_HOME") {
        return Some(PathBuf::from(home).join("config.toml"));
    }

    #[cfg(target_os = "windows")]
    {
        if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
            return Some(
                PathBuf::from(local_app_data)
                    .join("Volta")
                    .join("config.toml"),
            );
        }
        None
    }

    #[cfg(not(target_os = "windows"))]
    {
        if let Some(home) = dirs::home_dir() {
            return Some(home.join(".volta").join("config.toml"));
        }
        None
    }
}

// Read config.toml if it exists
fn read_config_file() -> Result<Option<Settings>, VoltaError> {
    if let Some(path) = config_file_path() {
        if path.exists() {
            let contents = fs::read_to_string(&path).map_err(|err| {
                VoltaError::from(ErrorKind::ConfigError {
                    message: format!("Failed to read {}: {}", path.display(), err),
                })
            })?;

            let cfg: Settings = toml::from_str(&contents).map_err(|err| {
                VoltaError::from(ErrorKind::ConfigError {
                    message: format!("Failed to parse {}: {}", path.display(), err),
                })
            })?;

            return Ok(Some(cfg));
        }
    }
    Ok(None)
}

// Apply env variable overrides on top of loaded settings
fn apply_env_overrides(settings: &mut Settings) {
    if let Ok(node_mirror) = std::env::var("VOLTA_NODE_DIST_BASE") {
        let node_mirror = node_mirror.trim();
        if !node_mirror.is_empty() {
            settings.mirrors.node = Some(node_mirror.to_string());
        }
    }
    if let Ok(registry_mirror) = std::env::var("VOLTA_REGISTRY_BASE") {
        let registry_mirror = registry_mirror.trim();
        if !registry_mirror.is_empty() {
            settings.mirrors.registry = Some(registry_mirror.to_string());
        }
    }
}

pub fn load_settings() -> Result<Settings, VoltaError> {
    // 1) start from defaults
    let mut settings = Settings::default();

    // 2) replace with config file (serde default fills missing fields)
    if let Some(cfg) = read_config_file()? {
        settings = cfg;
    }

    // 3) overlay env vars
    apply_env_overrides(&mut settings);

    Ok(settings)
}
