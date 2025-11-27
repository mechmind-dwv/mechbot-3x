pub mod api;
pub mod config;
pub mod control;
pub mod navigation;
pub mod robot;
pub mod sensors;
pub mod vision;

// Re-export para fácil acceso
pub use api::{ApiServer, WebSocketServer};
pub use config::Config;
pub use control::{ControlInput, ControlSystem, RobotState};
pub use navigation::NavigationController;
pub use robot::Robot;
pub use sensors::{Camera, Lidar, SensorData, SensorManager, IMU};
pub use vision::VisionProcessor;

// Función de inicialización del sistema
pub async fn initialize_system(
    config: Config,
) -> anyhow::Result<(
    sensors::Lidar,
    sensors::Camera,
    sensors::IMU,
    NavigationController,
    VisionProcessor,
    ApiServer,
)> {
    // Usar valores por defecto si los campos Option son None
    let lidar_port = config
        .sensors
        .lidar_port
        .clone()
        .unwrap_or_else(|| "/dev/ttyUSB0".to_string());
    let lidar_baudrate = config.sensors.lidar_baudrate.unwrap_or(115200);
    let camera_index = config.sensors.camera_index.unwrap_or(0);
    let imu_address = config.sensors.imu_i2c_address.unwrap_or(0x68);
    let rest_port = config.api.rest_port.unwrap_or(8080);

    // Inicializar componentes con valores por defecto
    let mut lidar = sensors::Lidar::new(sensors::LidarConfig {
        port: lidar_port,
        baudrate: lidar_baudrate,
        ..Default::default()
    });

    let mut camera = sensors::Camera::new(sensors::CameraConfig {
        device_path: format!("/dev/video{}", camera_index),
        ..Default::default()
    });

    let mut imu = sensors::IMU::new(sensors::IMUConfig {
        i2c_address: imu_address,
        ..Default::default()
    });

    // Inicializar sensores
    lidar.connect().await?;
    camera.connect().await?;
    imu.connect().await?;

    let navigation = NavigationController::new(navigation::NavigationConfig::default());

    // Inicializar visión
    let mut vision = VisionProcessor::new();
    vision.initialize().await?;

    let api_server = ApiServer::new(rest_port);

    Ok((lidar, camera, imu, navigation, vision, api_server))
}

// Implementaciones de traits necesarios
use std::fmt;

impl fmt::Debug for ApiServer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ApiServer(port: {})", self.port)
    }
}

// Implementaciones placeholder para sensores
impl sensors::Lidar {
    pub fn get_status(&self) -> sensors::SensorStatus {
        sensors::SensorStatus {
            connected: true,
            last_update: std::time::SystemTime::now(),
            error_count: 0,
            health: sensors::SensorHealth::Healthy,
        }
    }
}

impl sensors::Camera {
    pub fn get_status(&self) -> sensors::SensorStatus {
        sensors::SensorStatus {
            connected: true,
            last_update: std::time::SystemTime::now(),
            error_count: 0,
            health: sensors::SensorHealth::Healthy,
        }
    }
}

impl sensors::IMU {
    pub fn get_status(&self) -> sensors::SensorStatus {
        sensors::SensorStatus {
            connected: true,
            last_update: std::time::SystemTime::now(),
            error_count: 0,
            health: sensors::SensorHealth::Healthy,
        }
    }
}

impl ApiServer {
    pub fn is_running(&self) -> bool {
        true
    }

    pub fn stop(&self) {
        // Implementación placeholder
    }
}
