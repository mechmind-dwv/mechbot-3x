use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub robot: RobotConfig,
    pub sensors: SensorsConfig,
    pub navigation: NavigationConfig,
    pub api: ApiConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RobotConfig {
    pub name: String,
    pub model: String,
    pub version: String,
    pub max_speed: f64,
    pub max_acceleration: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorsConfig {
    pub lidar_port: Option<String>,
    pub lidar_baudrate: Option<u32>,
    pub camera_index: Option<u32>,
    pub imu_i2c_address: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationConfig {
    pub max_speed: f64,
    pub max_acceleration: f64,
    pub planning_frequency: u32,
    pub obstacle_distance_threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub rest_port: Option<u16>,
    pub websocket_port: Option<u16>,
    pub enable_cors: bool,
    pub api_key_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub output: String,
    pub max_file_size: String,
    pub rotate: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            robot: RobotConfig {
                name: "MechBot-3X".to_string(),
                model: "MB3X".to_string(),
                version: "3.0.0".to_string(),
                max_speed: 2.0,
                max_acceleration: 1.0,
            },
            sensors: SensorsConfig {
                lidar_port: Some("/dev/ttyUSB0".to_string()),
                lidar_baudrate: Some(115200),
                camera_index: Some(0),
                imu_i2c_address: Some(0x68),
            },
            navigation: NavigationConfig {
                max_speed: 2.0,
                max_acceleration: 1.0,
                planning_frequency: 10,
                obstacle_distance_threshold: 0.5,
            },
            api: ApiConfig {
                rest_port: Some(8080),
                websocket_port: Some(8081),
                enable_cors: true,
                api_key_required: true,
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                output: "logs/mechbot.log".to_string(),
                max_file_size: "10MB".to_string(),
                rotate: true,
            },
        }
    }
}

impl Config {
    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn to_file<P: AsRef<std::path::Path>>(&self, path: P) -> anyhow::Result<()> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.robot.max_speed <= 0.0 {
            errors.push("Robot max_speed must be positive".to_string());
        }

        if self.navigation.obstacle_distance_threshold <= 0.0 {
            errors.push("Obstacle distance threshold must be positive".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
