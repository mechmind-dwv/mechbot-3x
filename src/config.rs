use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub robot: RobotConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RobotConfig {
    pub name: String,
    pub model: String,
    pub version: String,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path))?;

        let config: Self = toml::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {}", path))?;

        Ok(config)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            robot: RobotConfig {
                name: "MechBot-3x-Fixed".to_string(),
                model: "MB3X".to_string(),
                version: "3.0.0".to_string(),
            },
        }
    }
}
