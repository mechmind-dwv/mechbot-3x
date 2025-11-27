use super::{Sensor, SensorStatus};
use anyhow::Result;

#[derive(Debug)]
pub struct Lidar {
    port: String,
    baud_rate: u32,
    status: SensorStatus,
    points_per_scan: usize,
}

impl Lidar {
    pub fn new(port: String, baud_rate: u32) -> Self {
        Self {
            port,
            baud_rate,
            status: SensorStatus {
                connected: false,
                last_update: std::time::SystemTime::now(),
                error_count: 0,
            },
            points_per_scan: 360,
        }
    }

    pub fn scan_environment(&mut self) -> Result<Vec<LidarPoint>> {
        // Simulación de escaneo LIDAR
        let mut points = Vec::new();
        for angle in 0..self.points_per_scan {
            let distance = 5.0 + (angle as f64).sin().abs() * 3.0; // Patrón simulado
            points.push(LidarPoint {
                angle: angle as f64,
                distance,
                intensity: 0.8,
            });
        }
        self.status.last_update = std::time::SystemTime::now();
        Ok(points)
    }

    pub fn get_obstacles(&self, points: &[LidarPoint], threshold: f64) -> Vec<Obstacle> {
        points
            .iter()
            .filter(|p| p.distance < threshold)
            .map(|p| Obstacle {
                position: (
                    p.angle.to_radians().cos() * p.distance,
                    p.angle.to_radians().sin() * p.distance,
                ),
                distance: p.distance,
            })
            .collect()
    }
}

impl Sensor for Lidar {
    fn initialize(&mut self) -> Result<()> {
        // Simular inicialización del LIDAR
        self.status.connected = true;
        self.status.error_count = 0;
        println!(
            "🟢 LIDAR inicializado en {} a {} baudios",
            self.port, self.baud_rate
        );
        Ok(())
    }

    fn read_data(&mut self) -> Result<Vec<u8>> {
        // Simular lectura de datos (en una implementación real, esto leería del puerto serie)
        let simulated_data = vec![0xAA, 0xBB, 0xCC, 0xDD];
        Ok(simulated_data)
    }

    fn get_status(&self) -> SensorStatus {
        self.status.clone()
    }
}

#[derive(Debug, Clone)]
pub struct LidarPoint {
    pub angle: f64,     // en grados
    pub distance: f64,  // en metros
    pub intensity: f64, // 0.0 a 1.0
}

#[derive(Debug)]
pub struct Obstacle {
    pub position: (f64, f64),
    pub distance: f64,
}
