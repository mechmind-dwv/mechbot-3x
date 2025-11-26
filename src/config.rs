use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub robot: RobotConfig,
    pub sensors: SensorsConfig,
    pub api: ApiConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RobotConfig {
    pub name: String,
    pub model: String,
    pub version: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SensorsConfig {
    pub lidar_port: Option<String>,
    pub camera_index: Option<u32>,
    pub sensor_threshold: f64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ApiConfig {
    pub rest_port: u16,
    pub websocket_port: u16,
    pub api_key: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LoggingConfig {
    pub level: String,
    pub output: String,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("No se pudo leer el archivo de configuración: {}", path))?;
        
        let config: Self = toml::from_str(&content)
            .with_context(|| format!("Error parseando configuración: {}", path))?;
        
        Ok(config)
    }

    pub fn save_to_file(&self, path: &str) -> Result<()> {
        let toml_content = toml::to_string_pretty(self)
            .context("Error serializando configuración a TOML")?;
        
        fs::write(path, toml_content)
            .with_context(|| format!("Error guardando configuración en: {}", path))?;
        
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            robot: RobotConfig {
                name: "MechBot-3x-Default".to_string(),
                model: "MB3X".to_string(),
                version: "3.0.0".to_string(),
            },
            sensors: SensorsConfig {
                lidar_port: Some("/dev/ttyUSB0".to_string()),
                camera_index: Some(0),
                sensor_threshold: 100.0,
            },
            api: ApiConfig {
                rest_port: 8080,
                websocket_port: 8081,
                api_key: None,
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                output: "console".to_string(),
            },
        }
    }
}
