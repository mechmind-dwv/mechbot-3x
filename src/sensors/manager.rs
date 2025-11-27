use super::drivers::{Camera, Lidar, SensorDriver, IMU};
use super::{CameraData, IMUData, LidarData, SensorData};
use anyhow::Result;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;

pub struct SensorManager {
    lidar: Option<Lidar>,
    imu: Option<IMU>,
    camera: Option<Camera>,
    data_buffer: Vec<SensorData>,
    max_buffer_size: usize,
}

impl SensorManager {
    pub fn new() -> Self {
        Self {
            lidar: None,
            imu: None,
            camera: None,
            data_buffer: Vec::new(),
            max_buffer_size: 1000,
        }
    }

    pub async fn initialize_all(&mut self) -> Result<()> {
        // Inicializar LIDAR si está configurado
        if let Some(lidar) = &mut self.lidar {
            lidar.connect().await?;
        }

        // Inicializar IMU si está configurado
        if let Some(imu) = &mut self.imu {
            imu.connect().await?;
            imu.calibrate().await?;
        }

        // Inicializar cámara si está configurado
        if let Some(camera) = &mut self.camera {
            camera.connect().await?;
        }

        Ok(())
    }

    pub async fn read_all_sensors(&mut self) -> Result<SensorData> {
        let mut sensor_data = SensorData::default();

        // Leer LIDAR
        if let Some(lidar) = &mut self.lidar {
            if let Ok(lidar_data) = lidar.scan().await {
                sensor_data.lidar = Some(lidar_data);
            }
        }

        // Leer IMU
        if let Some(imu) = &mut self.imu {
            if let Ok(imu_data) = imu.read_data().await {
                sensor_data.imu = Some(imu_data);
            }
        }

        // Leer cámara
        if let Some(camera) = &mut self.camera {
            if let Ok(camera_data) = camera.capture_frame().await {
                sensor_data.camera = Some(camera_data);
            }
        }

        // Guardar en buffer
        self.data_buffer.push(sensor_data.clone());
        if self.data_buffer.len() > self.max_buffer_size {
            self.data_buffer.remove(0);
        }

        Ok(sensor_data)
    }

    // Getters
    pub fn set_lidar(&mut self, lidar: Lidar) {
        self.lidar = Some(lidar);
    }

    pub fn set_imu(&mut self, imu: IMU) {
        self.imu = Some(imu);
    }

    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = Some(camera);
    }

    pub fn get_recent_data(&self, count: usize) -> &[SensorData] {
        let start = if self.data_buffer.len() > count {
            self.data_buffer.len() - count
        } else {
            0
        };
        &self.data_buffer[start..]
    }

    pub fn get_sensor_status(&self) -> HashMap<String, bool> {
        let mut status = HashMap::new();

        status.insert(
            "lidar".to_string(),
            self.lidar
                .as_ref()
                .map(|l| l.is_connected())
                .unwrap_or(false),
        );
        status.insert(
            "imu".to_string(),
            self.imu.as_ref().map(|i| i.is_connected()).unwrap_or(false),
        );
        status.insert(
            "camera".to_string(),
            self.camera
                .as_ref()
                .map(|c| c.is_connected())
                .unwrap_or(false),
        );

        status
    }
}

impl Default for SensorManager {
    fn default() -> Self {
        Self::new()
    }
}
