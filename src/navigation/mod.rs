pub mod pathfinding;
pub mod slam;

use crate::control::{ControlInput, RobotState};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationConfig {
    pub pathfinding: PathfindingConfig,
    pub slam: SLAMConfig,
    pub control: crate::control::ControlConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathfindingConfig {
    pub algorithm: PathfindingAlgorithm,
    pub grid_resolution: f64,
    pub safety_margin: f64,
    pub max_planning_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SLAMConfig {
    pub map_resolution: f64,
    pub map_size: (usize, usize),
    pub particle_count: usize,
    pub sensor_range: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PathfindingAlgorithm {
    AStar,
    RRT,
    RRTStar,
    Dijkstra,
}

impl Default for NavigationConfig {
    fn default() -> Self {
        Self {
            pathfinding: PathfindingConfig {
                algorithm: PathfindingAlgorithm::AStar,
                grid_resolution: 0.05, // 5cm
                safety_margin: 0.2,    // 20cm
                max_planning_time_ms: 1000,
            },
            slam: SLAMConfig {
                map_resolution: 0.05,   // 5cm
                map_size: (1000, 1000), // 50x50 meters
                particle_count: 1000,
                sensor_range: 10.0, // 10 meters
            },
            control: crate::control::ControlConfig::default(),
        }
    }
}

pub struct NavigationController {
    path_planner: pathfinding::PathPlanner,
    slam_engine: slam::SLAMEngine,
    current_path: Option<Vec<RobotState>>,
    current_goal: Option<RobotState>,
    config: NavigationConfig,
}

impl NavigationController {
    pub fn new(config: NavigationConfig) -> Self {
        Self {
            path_planner: pathfinding::PathPlanner::new(config.pathfinding.clone()),
            slam_engine: slam::SLAMEngine::new(config.slam.clone()),
            current_path: None,
            current_goal: None,
            config,
        }
    }

    pub async fn navigate_to_pose(
        &mut self,
        target_pose: RobotState,
        current_pose: RobotState,
        sensor_data: &SensorData,
    ) -> Result<ControlInput, String> {
        // Actualizar SLAM con datos de sensores
        self.slam_engine.update(current_pose, sensor_data).await?;

        // Obtener mapa actualizado
        let map = self.slam_engine.get_map();

        // Planificar ruta si no hay una actual o el objetivo cambió
        if self.should_replan(&current_pose, &target_pose) {
            match self
                .path_planner
                .plan_path(&current_pose, &target_pose, map)
                .await
            {
src/navigation/mod.rs                Ok(path) => {
                    self.current_path = Some(path);
                    self.current_goal = Some(target_pose);
                }
                Err(e) => return Err(format!("Path planning failed: {}", e)),
            }
        }

        // Seguir la ruta planificada
        if let Some(ref path) = self.current_path {
            self.follow_path(&current_pose, path)
        } else {
            Err("No path available".to_string())
        }
    }

    pub async fn explore_unknown_area(
        &mut self,
        current_pose: RobotState,
        sensor_data: &SensorData,
    ) -> Result<ControlInput, String> {
        // Usar RRT para exploración en espacio desconocido
        let frontier = self.slam_engine.get_exploration_frontier();
        let exploration_target = self
            .path_planner
            .plan_exploration(&current_pose, &frontier, self.slam_engine.get_map())
            .await?;

        self.navigate_to_pose(exploration_target, current_pose, sensor_data)
            .await
    }

    fn should_replan(&self, current_pose: &RobotState, target_pose: &RobotState) -> bool {
        self.current_path.is_none()
            || self.current_goal.as_ref() != Some(target_pose)
            || self.is_path_blocked(current_pose)
    }

    fn is_path_blocked(&self, current_pose: &RobotState) -> bool {
        // Verificar si el camino actual está bloqueado por obstáculos
        if let Some(ref path) = self.current_path {
            // Verificar los próximos puntos del camino
            for point in path.iter().take(10) {
                if self.slam_engine.is_occupied(point.x, point.y) {
                    return true;
                }
            }
        }
        false
    }

    fn follow_path(
        &self,
        current_pose: &RobotState,
        path: &[RobotState],
    ) -> Result<ControlInput, String> {
        if path.is_empty() {
            return Err("Empty path".to_string());
        }

        // Encontrar el punto más cercano en el camino
        let lookahead_point = self.find_lookahead_point(current_pose, path);

        // Calcular error de seguimiento
        let dx = lookahead_point.x - current_pose.x;
        let dy = lookahead_point.y - current_pose.y;
        let target_heading = dy.atan2(dx);

        let heading_error = target_heading - current_pose.theta;

        // Control simple para seguimiento de camino
        Ok(ControlInput::new(
            (dx * dx + dy * dy).sqrt().min(0.5), // Velocidad lineal limitada
            2.0 * heading_error,                 // Control proporcional de orientación
        ))
    }

    fn find_lookahead_point(&self, current_pose: &RobotState, path: &[RobotState]) -> RobotState {
        let lookahead_distance = 0.5; // 50cm de lookahead

        for i in 0..path.len() - 1 {
            let segment_start = &path[i];
            let segment_end = &path[i + 1];

            // Calcular distancia a este segmento
            let projection =
                self.project_point_to_segment(current_pose, segment_start, segment_end);
            let distance_to_projection = current_pose.distance_to(&projection);

            if distance_to_projection <= lookahead_distance {
                // Encontrar punto en el segmento a la distancia de lookahead
                return self.point_at_distance(segment_start, segment_end, lookahead_distance);
            }
        }

        // Si no se encuentra, usar el último punto
        path.last().cloned().unwrap_or_else(|| current_pose.clone())
    }

    fn project_point_to_segment(
        &self,
        point: &RobotState,
        start: &RobotState,
        end: &RobotState,
    ) -> RobotState {
        let dx = end.x - start.x;
        let dy = end.y - start.y;
        let segment_length_sq = dx * dx + dy * dy;

        if segment_length_sq == 0.0 {
            return start.clone();
        }

        let t = ((point.x - start.x) * dx + (point.y - start.y) * dy) / segment_length_sq;
        let t = t.clamp(0.0, 1.0);

        RobotState::new(
            start.x + t * dx,
            start.y + t * dy,
            start.theta + t * (end.theta - start.theta),
        )
    }

    fn point_at_distance(&self, start: &RobotState, end: &RobotState, distance: f64) -> RobotState {
        let dx = end.x - start.x;
        let dy = end.y - start.y;
        let segment_length = (dx * dx + dy * dy).sqrt();

        if segment_length == 0.0 {
            return start.clone();
        }

        let t = distance / segment_length;
        let t = t.clamp(0.0, 1.0);

        RobotState::new(
            start.x + t * dx,
            start.y + t * dy,
            start.theta + t * (end.theta - start.theta),
        )
    }

    pub fn get_current_path(&self) -> Option<&Vec<RobotState>> {
        self.current_path.as_ref()
    }

    pub fn get_map(&self) -> &slam::OccupancyGrid {
        self.slam_engine.get_map()
    }

    pub fn get_pose_estimate(&self) -> RobotState {
        self.slam_engine.get_pose_estimate()
    }
}

#[derive(Debug, Clone)]
pub struct SensorData {
    pub lidar_scan: Vec<(f64, f64)>, // (distance, angle)
    pub odometry: (f64, f64, f64),   // (dx, dy, dtheta)
    pub timestamp: f64,
}

impl Default for NavigationController {
    fn default() -> Self {
        Self::new(NavigationConfig::default())
    }
}
// ... tu código existente ...

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_navigation_controller_creation() {
        let config = NavigationConfig::default();
        let controller = NavigationController::new(config);
        
        assert!(controller.current_path.is_none());
        assert!(controller.current_goal.is_none());
    }

    #[test]
    fn test_bounding_box_operations() {
        let bbox1 = BoundingBox::new(0.0, 0.0, 10.0, 10.0);
        let bbox2 = BoundingBox::new(5.0, 5.0, 10.0, 10.0);
        
        assert_eq!(bbox1.area(), 100.0);
        assert!(bbox1.intersection(&bbox2) > 0.0);
        assert!(bbox1.iou(&bbox2) > 0.0);
    }

    #[test]
    fn test_pose_operations() {
        let pose1 = Pose { x: 0.0, y: 0.0, theta: 0.0 };
        let pose2 = Pose { x: 3.0, y: 4.0, theta: 0.0 };
        
        // Test distancia (deberías implementar este método)
        // assert_eq!(pose1.distance_to(&pose2), 5.0);
    }
}

/// Calcula la ruta óptima entre dos puntos
///
/// # Arguments
/// * `start` - Punto de inicio  
/// * `goal` - Punto destino
/// * `obstacles` - Lista de obstáculos a evitar
///
/// # Returns
/// `Result<Path, NavigationError>` - Ruta calculada o error
///
/// # Examples
/// ```
/// use mechbot_3x::navigation::{PathPlanner, Point, Obstacle};
/// 
/// let planner = PathPlanner::new();
/// let start = Point::new(0.0, 0.0);
/// let goal = Point::new(10.0, 10.0);
/// let obstacles = vec![];
/// 
/// match planner.calculate_path(start, goal, &obstacles) {
///     Ok(path) => println!("Ruta encontrada con {} puntos", path.len()),
///     Err(e) => eprintln!("Error: {}", e),
/// }
/// ```
///
/// # Errors
/// Devuelve `NavigationError::NoPath` si no se encuentra ruta
pub fn calculate_path(
    start: Point, 
    goal: Point, 
    obstacles: &[Obstacle]
) -> Result<Vec<Point>, NavigationError> {
    // Implementación básica por ahora
    let mut path = Vec::new();
    path.push(start);
    path.push(goal);
    Ok(path)
}

#[derive(Debug)]
pub struct Obstacle {
    pub position: Point,
    pub radius: f64,
}

#[derive(Debug)]
pub enum NavigationError {
    NoPath,
    InvalidInput,
    ObstacleCollision,
}

impl std::fmt::Display for NavigationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NavigationError::NoPath => write!(f, "No se pudo encontrar una ruta"),
            NavigationError::InvalidInput => write!(f, "Entrada inválida"),
            NavigationError::ObstacleCollision => write!(f, "Colisión con obstáculo detectada"),
        }
    }
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl BoundingBox {
    pub fn area(&self) -> f64 {
        self.width * self.height
    }

    pub fn intersection(&self, other: &BoundingBox) -> f64 {
        let x1 = self.x.max(other.x);
        let y1 = self.y.max(other.y);
        let x2 = (self.x + self.width).min(other.x + other.width);
        let y2 = (self.y + self.height).min(other.y + other.height);

        if x2 > x1 && y2 > y1 {
            (x2 - x1) * (y2 - y1)
        } else {
            0.0
        }
    }

    pub fn iou(&self, other: &BoundingBox) -> f64 {
        let intersection = self.intersection(other);
        let union = self.area() + other.area() - intersection;
        
        if union > 0.0 {
            intersection / union
        } else {
            0.0
        }
    }
}
i
mpl std::error::Error for NavigationError {}
