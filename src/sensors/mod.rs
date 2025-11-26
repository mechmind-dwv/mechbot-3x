pub mod camera;
pub mod imu;
pub mod lidar;

use anyhow::Result;

// Traits comunes para todos los sensores
pub trait Sensor {
    fn initialize(&mut self) -> Result<()>;
    fn read_data(&mut self) -> Result<Vec<u8>>;
    fn get_status(&self) -> SensorStatus;
}

#[derive(Debug, Clone)]
pub struct SensorStatus {
    pub connected: bool,
    pub last_update: std::time::SystemTime,
    pub error_count: u32,
}

// Función de utilidad para procesamiento de datos de sensores
pub fn filter_sensor_data(data: &[f64], filter_type: &str) -> Vec<f64> {
    match filter_type {
        "kalman" => kalman_filter(data),
        "moving_average" => moving_average(data),
        _ => data.to_vec(),
    }
}

fn kalman_filter(data: &[f64]) -> Vec<f64> {
    // Implementación básica de filtro Kalman
    data.iter().map(|&x| x * 0.95).collect()
}

fn moving_average(data: &[f64]) -> Vec<f64> {
    if data.len() < 3 {
        return data.to_vec();
    }
    data.windows(3)
        .map(|w| w.iter().sum::<f64>() / 3.0)
        .collect()
}

// Re-export para fácil acceso
pub use camera::Camera;
pub use imu::IMU;
pub use lidar::Lidar;
