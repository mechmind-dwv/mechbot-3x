use anyhow::Result;

#[derive(Debug)]
pub struct SLAM {
    map: Vec<Vec<f64>>, // Grid map
    resolution: f64,    // metros por celda
    origin: (f64, f64), // posición del origen del mapa
}

impl SLAM {
    pub fn new(resolution: f64, width: usize, height: usize) -> Self {
        Self {
            map: vec![vec![0.0; width]; height],
            resolution,
            origin: (0.0, 0.0),
        }
    }

    pub fn update_with_lidar(
        &mut self,
        _position: (f64, f64),
        points: &[(f64, f64)],
    ) -> Result<()> {
        // Actualizar mapa con datos LIDAR
        for &point in points {
            let map_x = ((point.0 - self.origin.0) / self.resolution) as usize;
            let map_y = ((point.1 - self.origin.1) / self.resolution) as usize;

            if map_x < self.map[0].len() && map_y < self.map.len() {
                self.map[map_y][map_x] = 1.0; // Obstáculo
            }
        }
        Ok(())
    }

    pub fn get_occupancy_grid(&self) -> &Vec<Vec<f64>> {
        &self.map
    }

    pub fn estimate_position(&self, _sensor_data: &[f64]) -> (f64, f64) {
        // Estimación básica de posición
        (0.0, 0.0) // Posición estimada
    }
}

impl Default for SLAM {
    fn default() -> Self {
        Self::new(0.1, 100, 100) // 10cm resolución, 10x10 metros
    }
}
