use crate::navigation::Point;

#[derive(Debug, Clone)]
pub struct OccupancyGrid {
    pub resolution: f64,
    pub width: usize,
    pub height: usize,
    pub data: Vec<f64>,
}

impl OccupancyGrid {
    pub fn new(width: usize, height: usize, resolution: f64) -> Self {
        Self {
            resolution,
            width,
            height,
            data: vec![0.5; width * height], // 0.5 = desconocido
        }
    }
    
    pub fn update_from_lidar(&mut self, points: &[Point], robot_pose: (f64, f64, f64)) {
        // Actualizar grid con datos LIDAR
        println!("Actualizando grid de ocupación con {} puntos LIDAR", points.len());
    }
}

pub enum SLAMAlgorithm {
    EKFSLAM,
    FastSLAM,
    ORBSLAM,
}

pub fn run_slam(algorithm: SLAMAlgorithm, sensor_data: &[f64]) -> Result<(), String> {
    match algorithm {
        SLAMAlgorithm::EKFSLAM => run_ekf_slam(sensor_data),
        SLAMAlgorithm::FastSLAM => run_fast_slam(sensor_data),
        SLAMAlgorithm::ORBSLAM => run_orb_slam(sensor_data),
    }
}

fn run_ekf_slam(_sensor_data: &[f64]) -> Result<(), String> {
    println!("Ejecutando EKF-SLAM");
    Ok(())
}

fn run_fast_slam(_sensor_data: &[f64]) -> Result<(), String> {
    println!("Ejecutando FastSLAM");
    Ok(())
}

fn run_orb_slam(_sensor_data: &[f64]) -> Result<(), String> {
    println!("Ejecutando ORB-SLAM");
    Ok(())
}
