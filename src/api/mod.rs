pub mod rest;
pub mod websocket;

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct ApiServer {
    pub port: u16,
    is_running: bool,
}

impl ApiServer {
    pub fn new(port: u16) -> Self {
        Self {
            port,
            is_running: false,
        }
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn stop(&self) {
        log::info!("🛑 Deteniendo servidor API...");
    }

    pub async fn start(&mut self) -> anyhow::Result<()> {
        log::info!("🚀 Iniciando servidor API en puerto {}", self.port);

        // Iniciar servidor REST
        let rest_handle = tokio::spawn(rest::start_rest_server(self.port));

        // Iniciar servidor WebSocket
        let websocket_handle = tokio::spawn(websocket::start_websocket_server(self.port + 1));

        self.is_running = true;
        log::info!("✅ Servidores API iniciados:");
        log::info!("   - REST: http://localhost:{}", self.port);
        log::info!("   - WebSocket: ws://localhost:{}", self.port + 1);

        // Mantener los servidores corriendo
        tokio::select! {
            _ = rest_handle => {},
            _ = websocket_handle => {},
        }

        Ok(())
    }
}

// Estructuras de datos para la API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RobotStatus {
    pub name: String,
    pub version: String,
    pub status: RobotState,
    pub position: Position,
    pub battery_level: f64,
    pub uptime: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RobotState {
    pub state: String,
    pub mode: String,
    pub health: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub theta: f64,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveCommand {
    pub x: f64,
    pub y: f64,
    pub speed: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorData {
    pub lidar: LidarData,
    pub imu: IMUData,
    pub camera: CameraData,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LidarData {
    pub points: Vec<Point>,
    pub min_distance: f64,
    pub max_distance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IMUData {
    pub acceleration: Vector3,
    pub gyroscope: Vector3,
    pub magnetometer: Vector3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraData {
    pub frame_id: String,
    pub resolution: (u32, u32),
    pub objects_detected: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapData {
    pub resolution: f64,
    pub width: u32,
    pub height: u32,
    pub origin: Point,
    pub data: Vec<u8>,
}

// Estado compartido de la aplicación
#[derive(Debug, Clone)]
pub struct AppState {
    pub robot_status: RobotStatus,
    pub last_sensor_data: SensorData,
    pub map_data: MapData,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            robot_status: RobotStatus {
                name: "MechBot-3x".to_string(),
                version: "0.1.0".to_string(),
                status: RobotState {
                    state: "ready".to_string(),
                    mode: "autonomous".to_string(),
                    health: "excellent".to_string(),
                },
                position: Position {
                    x: 0.0,
                    y: 0.0,
                    theta: 0.0,
                    timestamp: chrono::Utc::now().to_rfc3339(),
                },
                battery_level: 95.5,
                uptime: 0,
            },
            last_sensor_data: SensorData {
                lidar: LidarData {
                    points: vec![],
                    min_distance: 0.0,
                    max_distance: 0.0,
                },
                imu: IMUData {
                    acceleration: Vector3 {
                        x: 0.0,
                        y: 0.0,
                        z: 9.81,
                    },
                    gyroscope: Vector3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    magnetometer: Vector3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                },
                camera: CameraData {
                    frame_id: "frame_001".to_string(),
                    resolution: (640, 480),
                    objects_detected: 0,
                },
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
            map_data: MapData {
                resolution: 0.05,
                width: 100,
                height: 100,
                origin: Point { x: -2.5, y: -2.5 },
                data: vec![0; 100 * 100],
            },
        }
    }
}
// ... tu código existente ...

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;

    #[test]
    fn test_robot_status_creation() {
        let status = RobotStatus {
            name: "TestBot".to_string(),
            version: "1.0.0".to_string(),
            status: RobotState {
                state: "ready".to_string(),
                mode: "autonomous".to_string(),
                health: "excellent".to_string(),
            },
            position: Position {
                x: 1.0,
                y: 2.0,
                theta: 0.0,
                timestamp: "2024-01-01T00:00:00Z".to_string(),
            },
            battery_level: 100.0,
            uptime: 3600,
        };

        assert_eq!(status.name, "TestBot");
        assert_eq!(status.battery_level, 100.0);
    }

    #[tokio::test]
    async fn test_move_command_deserialization() {
        let json_data = r#"
        {
            "x": 10.5,
            "y": 20.3,
            "speed": 1.5
        }
        "#;

        let command: Result<MoveCommand, _> = serde_json::from_str(json_data);
        assert!(command.is_ok());
        
        let cmd = command.unwrap();
        assert_eq!(cmd.x, 10.5);
        assert_eq!(cmd.speed, 1.5);
    }
}
