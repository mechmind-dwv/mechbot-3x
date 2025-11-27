use super::SLAMConfig;
use crate::control::RobotState;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SLAMEngine {
    mapper: OccupancyGridMapper,
    localizer: ParticleFilterLocalizer,
    config: SLAMConfig,
    pose_history: Vec<RobotState>,
}

impl SLAMEngine {
    pub fn new(config: SLAMConfig) -> Self {
        Self {
            mapper: OccupancyGridMapper::new(config.map_size, config.map_resolution),
            localizer: ParticleFilterLocalizer::new(config.particle_count),
            config,
            pose_history: Vec::new(),
        }
    }

    pub async fn update(
        &mut self,
        odometry_pose: RobotState,
        sensor_data: &super::SensorData,
    ) -> Result<(), String> {
        // Paso de predicción del filtro de partículas
        self.localizer.predict(&sensor_data.odometry);

        // Paso de corrección usando datos LIDAR
        let weights = self.calculate_particle_weights(&sensor_data.lidar_scan);
        self.localizer.resample(&weights);

        // Obtener estimación de pose
        let estimated_pose = self.localizer.get_estimated_pose();

        // Actualizar mapa con la pose estimada y datos LIDAR
        self.mapper
            .update_map(&estimated_pose, &sensor_data.lidar_scan);

        // Guardar historial de poses
        self.pose_history.push(estimated_pose.clone());

        Ok(())
    }

    fn calculate_particle_weights(&self, lidar_scan: &[(f64, f64)]) -> Vec<f64> {
        self.localizer
            .get_particles()
            .iter()
            .map(|particle| self.calculate_particle_likelihood(particle, lidar_scan))
            .collect()
    }

    fn calculate_particle_likelihood(
        &self,
        particle: &RobotState,
        lidar_scan: &[(f64, f64)],
    ) -> f64 {
        let mut likelihood = 1.0;

        for &(distance, angle) in lidar_scan {
            let global_angle = particle.theta + angle;
            let expected_distance = self.mapper.ray_cast(particle, global_angle);

            if expected_distance.is_finite() {
                // Modelo de sensor: probabilidad gaussiana
                let error = (distance - expected_distance).abs();
                let sensor_std_dev = 0.1; // 10cm de desviación estándar
                let probability = (-error.powi(2) / (2.0 * sensor_std_dev.powi(2))).exp();
                likelihood *= probability;
            }
        }

        likelihood
    }

    pub fn get_map(&self) -> &OccupancyGrid {
        &self.mapper.grid
    }

    pub fn get_pose_estimate(&self) -> RobotState {
        self.localizer.get_estimated_pose()
    }

    pub fn is_occupied(&self, x: f64, y: f64) -> bool {
        self.mapper.grid.is_occupied(x, y, 0.0)
    }

    pub fn get_exploration_frontier(&self) -> Vec<RobotState> {
        self.mapper.find_frontiers(&self.get_pose_estimate())
    }
}

#[derive(Debug, Clone)]
pub struct OccupancyGridMapper {
    pub grid: OccupancyGrid,
    resolution: f64,
}

impl OccupancyGridMapper {
    pub fn new(size: (usize, usize), resolution: f64) -> Self {
        Self {
            grid: OccupancyGrid::new(size.0, size.1, resolution),
            resolution,
        }
    }

    pub fn update_map(&mut self, robot_pose: &RobotState, lidar_scan: &[(f64, f64)]) {
        for &(distance, angle) in lidar_scan {
            if distance > self.grid.max_range {
                continue;
            }

            let global_angle = robot_pose.theta + angle;

            // Coordenadas globales del punto de impacto del rayo
            let hit_x = robot_pose.x + distance * global_angle.cos();
            let hit_y = robot_pose.y + distance * global_angle.sin();

            // Actualizar celdas a lo largo del rayo como libres
            self.update_ray(robot_pose, hit_x, hit_y, false);

            // Actualizar celda de impacto como ocupada
            if let Some((cell_x, cell_y)) = self.grid.world_to_grid(hit_x, hit_y) {
                self.grid.update_cell(cell_x, cell_y, true);
            }
        }
    }

    fn update_ray(&mut self, start: &RobotState, end_x: f64, end_y: f64, occupied: bool) {
        let dx = end_x - start.x;
        let dy = end_y - start.y;
        let distance = (dx * dx + dy * dy).sqrt();
        let steps = (distance / self.resolution).ceil() as usize;

        for i in 0..steps {
            let t = i as f64 / steps as f64;
            let x = start.x + t * dx;
            let y = start.y + t * dy;

            if let Some((cell_x, cell_y)) = self.grid.world_to_grid(x, y) {
                self.grid.update_cell(cell_x, cell_y, occupied);
            }
        }
    }

    pub fn find_frontiers(&self, __current_pose: &RobotState) -> Vec<RobotState> {
        let mut frontiers = Vec::new();
        let unknown_value = 0.5; // Valor que representa "desconocido" en el mapa de ocupación

        for x in 1..self.grid.width - 1 {
            for y in 1..self.grid.height - 1 {
                if self.grid.get(x, y) == unknown_value {
                    // Verificar si esta celda desconocida está al lado de una celda libre
                    if self.is_frontier_cell(x, y) {
                        let (world_x, world_y) = self.grid.grid_to_world(x, y);
                        frontiers.push(RobotState::new(world_x, world_y, 0.0));
                    }
                }
            }
        }

        // Ordenar por distancia a la pose actual
        frontiers.sort_by(|a, b| {
            _current_pose
                .distance_to(a)
                .partial_cmp(&_current_pose.distance_to(b))
                .unwrap()
        });

        frontiers
    }

    fn is_frontier_cell(&self, x: usize, y: usize) -> bool {
        let neighbors = [
            (x.wrapping_sub(1), y),
            (x + 1, y),
            (x, y.wrapping_sub(1)),
            (x, y + 1),
        ];

        neighbors.iter().any(|&(nx, ny)| {
            self.grid.get(nx, ny) < 0.3 // Celda libre
        })
    }
}

#[derive(Debug, Clone)]
pub struct OccupancyGrid {
    width: usize,
    height: usize,
    resolution: f64,
    data: Vec<f64>,
    origin_x: f64,
    origin_y: f64,
    pub max_range: f64,
}

impl OccupancyGrid {
    pub fn new(width: usize, height: usize, resolution: f64) -> Self {
        Self {
            width,
            height,
            resolution,
            data: vec![0.5; width * height], // 0.5 = desconocido
            origin_x: -(width as f64 * resolution) / 2.0,
            origin_y: -(height as f64 * resolution) / 2.0,
            max_range: 10.0, // 10 metros de rango máximo
        }
    }

    pub fn update_cell(&mut self, x: usize, y: usize, occupied: bool) {
        if let Some(index) = self.grid_to_index(x, y) {
            // Actualización bayesiana simple
            let prior = self.data[index];
            let probability = if occupied { 0.7 } else { 0.3 }; // Probabilidades del sensor
            let new_value =
                (probability * prior) / (probability * prior + (1.0 - probability) * (1.0 - prior));
            self.data[index] = new_value.clamp(0.0, 1.0);
        }
    }

    pub fn get(&self, x: usize, y: usize) -> f64 {
        self.grid_to_index(x, y)
            .map(|index| self.data[index])
            .unwrap_or(0.5) // Desconocido si está fuera de los límites
    }

    pub fn is_occupied(&self, world_x: f64, world_y: f64, safety_margin: f64) -> bool {
        if let Some((x, y)) = self.world_to_grid(world_x, world_y) {
            // Verificar celdas dentro del margen de seguridad
            let margin_cells = (safety_margin / self.resolution).ceil() as i32;

            for dx in -margin_cells..=margin_cells {
                for dy in -margin_cells..=margin_cells {
                    let check_x = (x as i32 + dx) as usize;
                    let check_y = (y as i32 + dy) as usize;

                    if self.get(check_x, check_y) > 0.65 {
                        // Umbral de ocupación
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn world_to_grid(&self, world_x: f64, world_y: f64) -> Option<(usize, usize)> {
        let grid_x = ((world_x - self.origin_x) / self.resolution).round() as i32;
        let grid_y = ((world_y - self.origin_y) / self.resolution).round() as i32;

        if grid_x >= 0 && grid_x < self.width as i32 && grid_y >= 0 && grid_y < self.height as i32 {
            Some((grid_x as usize, grid_y as usize))
        } else {
            None
        }
    }

    pub fn grid_to_world(&self, grid_x: usize, grid_y: usize) -> (f64, f64) {
        (
            self.origin_x + grid_x as f64 * self.resolution,
            self.origin_y + grid_y as f64 * self.resolution,
        )
    }

    pub fn ray_cast(&self, start: &RobotState, angle: f64) -> f64 {
        let mut distance = 0.0;
        let step_size = self.resolution * 0.5; // Medio paso de resolución

        while distance < self.max_range {
            let x = start.x + distance * angle.cos();
            let y = start.y + distance * angle.sin();

            if self.is_occupied(x, y, 0.0) {
                return distance;
            }

            distance += step_size;
        }

        self.max_range
    }

    pub fn get_known_bounds(&self) -> MapBounds {
        MapBounds {
            min_x: self.origin_x,
            min_y: self.origin_y,
            width: self.width as f64 * self.resolution,
            height: self.height as f64 * self.resolution,
        }
    }

    fn grid_to_index(&self, x: usize, y: usize) -> Option<usize> {
        if x < self.width && y < self.height {
            Some(y * self.width + x)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
struct ParticleFilterLocalizer {
    particles: Vec<RobotState>,
    weights: Vec<f64>,
}

impl ParticleFilterLocalizer {
    pub fn new(count: usize) -> Self {
        let particles = (0..count)
            .map(|_| {
                RobotState::new(
                    rand::random::<f64>() * 2.0 - 1.0, // -1 to 1 meters
                    rand::random::<f64>() * 2.0 - 1.0,
                    rand::random::<f64>() * 2.0 * std::f64::consts::PI,
                )
            })
            .collect();

        Self {
            particles,
            weights: vec![1.0 / count as f64; count],
        }
    }

    pub fn predict(&mut self, odometry: &(f64, f64, f64)) {
        for particle in &mut self.particles {
            // Aplicar odometría con ruado
            let (dx, dy, dtheta) = odometry;
            let noise_std = 0.05; // 5cm de ruado en posición, 0.1 rad en orientación

            particle.x += dx + rand::random::<f64>() * noise_std;
            particle.y += dy + rand::random::<f64>() * noise_std;
            particle.theta += dtheta + rand::random::<f64>() * 0.1;

            // Normalizar orientación
            particle.theta = particle.theta % (2.0 * std::f64::consts::PI);
        }
    }

    pub fn resample(&mut self, weights: &[f64]) {
        let total_weight: f64 = weights.iter().sum();
        if total_weight == 0.0 {
            return;
        }

        let mut new_particles = Vec::new();
        let mut cumulative_weights: Vec<f64> = weights
            .iter()
            .scan(0.0, |acc, &w| {
                *acc += w / total_weight;
                Some(*acc)
            })
            .collect();

        for _ in 0..self.particles.len() {
            let random_value = rand::random::<f64>();
            let selected_index = cumulative_weights
                .iter()
                .position(|&w| w >= random_value)
                .unwrap_or(0);

            new_particles.push(self.particles[selected_index].clone());
        }

        self.particles = new_particles;
        // Reset weights
        let uniform_weight = 1.0 / self.particles.len() as f64;
        self.weights = vec![uniform_weight; self.particles.len()];
    }

    pub fn get_estimated_pose(&self) -> RobotState {
        // Promedio de todas las partículas
        let sum_x: f64 = self.particles.iter().map(|p| p.x).sum();
        let sum_y: f64 = self.particles.iter().map(|p| p.y).sum();

        // Promedio circular para la orientación
        let sum_cos: f64 = self.particles.iter().map(|p| p.theta.cos()).sum();
        let sum_sin: f64 = self.particles.iter().map(|p| p.theta.sin()).sum();
        let avg_theta = sum_sin.atan2(sum_cos);

        RobotState::new(
            sum_x / self.particles.len() as f64,
            sum_y / self.particles.len() as f64,
            avg_theta,
        )
    }

    pub fn get_particles(&self) -> &Vec<RobotState> {
        &self.particles
    }
}

#[derive(Debug, Clone)]
struct MapBounds {
    min_x: f64,
    min_y: f64,
    width: f64,
    height: f64,
}
