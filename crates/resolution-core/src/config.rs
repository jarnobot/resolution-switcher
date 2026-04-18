use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Resolution {
    pub w: u32,
    pub h: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub native: Resolution,
    pub custom: Resolution,
    pub hz: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub active_idx: usize,
    pub profiles: [Profile; 2],
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Could not find config directory")]
    NoConfigDir,
}

fn config_path() -> Result<PathBuf, ConfigError> {
    let dir = dirs::config_dir()
        .ok_or(ConfigError::NoConfigDir)?
        .join("ResolutionSwitcher");
    Ok(dir.join("profiles.json"))
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        let path = config_path()?;
        if !path.exists() {
            return Ok(Self::default());
        }
        let raw = std::fs::read_to_string(&path)?;
        let value: serde_json::Value = serde_json::from_str(&raw)?;
        // Fall back to defaults if the structure is malformed
        if value
            .get("profiles")
            .and_then(|p| p.as_array())
            .map(|a| a.len())
            != Some(2)
        {
            return Ok(Self::default());
        }
        Ok(serde_json::from_value(value)?)
    }

    pub fn save(&self) -> Result<(), ConfigError> {
        let path = config_path()?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(&path, json)?;
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            active_idx: 0,
            profiles: [
                Profile {
                    name: "Monitor A".into(),
                    native: Resolution { w: 2560, h: 1440 },
                    custom: Resolution { w: 1920, h: 1080 },
                    hz: 240,
                },
                Profile {
                    name: "Monitor B".into(),
                    native: Resolution { w: 3840, h: 2160 },
                    custom: Resolution { w: 1920, h: 1080 },
                    hz: 144,
                },
            ],
        }
    }
}
