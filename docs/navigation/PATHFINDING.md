# Sistema de Navegación y Pathfinding

## 🧭 Algoritmos Implementados

### A* (A Estrella)
```rust
pub struct AStarPlanner {
    grid: NavigationGrid,
    heuristic: HeuristicFunction,
}

impl AStarPlanner {
    pub fn plan_path(&self, start: Point, goal: Point) -> Result<Vec<Point>> {
        // Implementación A* optimizada
    }
}
```

### RRT (Rapidly-exploring Random Tree)
```rust
pub struct RRTPlanner {
    max_iterations: u32,
    step_size: f64,
    goal_bias: f64,
}

impl RRTPlanner {
    pub fn plan_in_unknown_space(&self, start: Pose, goal: Point) -> Result<Vec<Pose>>;
}
```

## 🗺️ SLAM (Simultaneous Localization and Mapping)
```rust
pub struct SLAMEngine {
    mapper: OccupancyGridMapper,
    localizer: ParticleFilterLocalizer,
}

impl SLAMEngine {
    pub async fn process_scan(&mut self, lidar_scan: &[Point], odometry: Odometry) -> Result<()>;
    pub fn get_map(&self) -> &OccupancyGrid;
    pub fn get_pose(&self) -> Pose;
}
```
