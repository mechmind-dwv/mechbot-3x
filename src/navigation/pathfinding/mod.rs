use super::{NavigationConfig, PathfindingAlgorithm, PathfindingConfig};
use crate::control::RobotState;
use serde::{Deserialize, Serialize};
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct PathPlanner {
    config: PathfindingConfig,
    a_star: AStarPlanner,
    rrt: RRTPlanner,
}

impl PathPlanner {
    pub fn new(config: PathfindingConfig) -> Self {
        Self {
            config: config.clone(),
            a_star: AStarPlanner::new(config.grid_resolution, config.safety_margin),
            rrt: RRTPlanner::new(1000, 0.5, 0.1), // max_iterations, step_size, goal_bias
        }
    }

    pub async fn plan_path(
        &self,
        start: &RobotState,
        goal: &RobotState,
        map: &super::slam::OccupancyGrid,
    ) -> Result<Vec<RobotState>, String> {
        match self.config.algorithm {
            PathfindingAlgorithm::AStar => self.a_star.plan_path(start, goal, map).await,
            PathfindingAlgorithm::RRT => self.rrt.plan_path(start, goal, map).await,
            PathfindingAlgorithm::RRTStar => self.rrt.plan_path_star(start, goal, map).await,
            PathfindingAlgorithm::Dijkstra => self.a_star.plan_dijkstra(start, goal, map).await,
        }
    }

    pub async fn plan_exploration(
        &self,
        start: &RobotState,
        frontier: &[RobotState],
        map: &super::slam::OccupancyGrid,
    ) -> Result<RobotState, String> {
        self.rrt.plan_exploration(start, frontier, map).await
    }
}

#[derive(Debug, Clone)]
pub struct AStarPlanner {
    grid_resolution: f64,
    safety_margin: f64,
}

impl AStarPlanner {
    pub fn new(grid_resolution: f64, safety_margin: f64) -> Self {
        Self {
            grid_resolution,
            safety_margin,
        }
    }

    pub async fn plan_path(
        &self,
        start: &RobotState,
        goal: &RobotState,
        map: &super::slam::OccupancyGrid,
    ) -> Result<Vec<RobotState>, String> {
        let start_node = self.pose_to_node(start);
        let goal_node = self.pose_to_node(goal);

        let mut open_set = BinaryHeap::new();
        let mut came_from = HashMap::new();
        let mut g_score = HashMap::new();
        let mut f_score = HashMap::new();

        open_set.push(HeapNode {
            node: start_node.clone(),
            f_score: self.heuristic(&start_node, &goal_node),
        });

        g_score.insert(start_node.clone(), 0.0);
        f_score.insert(start_node.clone(), self.heuristic(&start_node, &goal_node));

        while let Some(HeapNode { node: current, .. }) = open_set.pop() {
            if current == goal_node {
                return Ok(self.reconstruct_path(came_from, current));
            }

            for neighbor in self.get_neighbors(&current, map) {
                let tentative_g_score = g_score[&current] + self.distance(&current, &neighbor);

                if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&f64::INFINITY) {
                    came_from.insert(neighbor.clone(), current.clone());
                    g_score.insert(neighbor.clone(), tentative_g_score);
                    let new_f_score = tentative_g_score + self.heuristic(&neighbor, &goal_node);
                    f_score.insert(neighbor.clone(), new_f_score);

                    open_set.push(HeapNode {
                        node: neighbor.clone(),
                        f_score: new_f_score,
                    });
                }
            }
        }

        Err("No path found".to_string())
    }

    pub async fn plan_dijkstra(
        &self,
        start: &RobotState,
        goal: &RobotState,
        map: &super::slam::OccupancyGrid,
    ) -> Result<Vec<RobotState>, String> {
        // Dijkstra es A* con heurística cero
        let start_node = self.pose_to_node(start);
        let goal_node = self.pose_to_node(goal);

        let mut open_set = BinaryHeap::new();
        let mut came_from = HashMap::new();
        let mut g_score = HashMap::new();

        open_set.push(HeapNode {
            node: start_node.clone(),
            f_score: 0.0,
        });

        g_score.insert(start_node.clone(), 0.0);

        while let Some(HeapNode { node: current, .. }) = open_set.pop() {
            if current == goal_node {
                return Ok(self.reconstruct_path(came_from, current));
            }

            for neighbor in self.get_neighbors(&current, map) {
                let tentative_g_score = g_score[&current] + self.distance(&current, &neighbor);

                if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&f64::INFINITY) {
                    came_from.insert(neighbor.clone(), current.clone());
                    g_score.insert(neighbor.clone(), tentative_g_score);

                    open_set.push(HeapNode {
                        node: neighbor.clone(),
                        f_score: tentative_g_score, // Dijkstra usa g_score como f_score
                    });
                }
            }
        }

        Err("No path found".to_string())
    }

    fn pose_to_node(&self, pose: &RobotState) -> GridNode {
        GridNode {
            x: (pose.x / self.grid_resolution).round() as i32,
            y: (pose.y / self.grid_resolution).round() as i32,
        }
    }

    fn node_to_pose(&self, node: &GridNode) -> RobotState {
        RobotState::new(
            node.x as f64 * self.grid_resolution,
            node.y as f64 * self.grid_resolution,
            0.0, // La orientación se calculará durante el suavizado del camino
        )
    }

    fn heuristic(&self, a: &GridNode, b: &GridNode) -> f64 {
        // Distancia Euclidiana
        let dx = (a.x - b.x) as f64;
        let dy = (a.y - b.y) as f64;
        (dx * dx + dy * dy).sqrt()
    }

    fn distance(&self, a: &GridNode, b: &GridNode) -> f64 {
        self.heuristic(a, b)
    }

    fn get_neighbors(&self, node: &GridNode, map: &super::slam::OccupancyGrid) -> Vec<GridNode> {
        let mut neighbors = Vec::new();

        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let neighbor = GridNode {
                    x: node.x + dx,
                    y: node.y + dy,
                };

                // Verificar si la celda es transitable
                let world_x = neighbor.x as f64 * self.grid_resolution;
                let world_y = neighbor.y as f64 * self.grid_resolution;

                if !map.is_occupied(world_x, world_y, self.safety_margin) {
                    neighbors.push(neighbor);
                }
            }
        }

        neighbors
    }

    fn reconstruct_path(
        &self,
        came_from: HashMap<GridNode, GridNode>,
        mut current: GridNode,
    ) -> Vec<RobotState> {
        let mut path = vec![self.node_to_pose(&current)];

        while let Some(prev) = came_from.get(&current) {
            path.push(self.node_to_pose(prev));
            current = prev.clone();
        }

        path.reverse();
        self.smooth_path(path)
    }

    fn smooth_path(&self, path: Vec<RobotState>) -> Vec<RobotState> {
        if path.len() < 3 {
            return path;
        }

        let mut smoothed = Vec::new();
        smoothed.push(path[0].clone());

        for i in 1..path.len() - 1 {
            // Suavizado simple - mantener cada n-ésimo punto
            if i % 3 == 0 {
                smoothed.push(path[i].clone());
            }
        }

        smoothed.push(path.last().unwrap().clone());
        smoothed
    }
}

#[derive(Debug, Clone)]
pub struct RRTPlanner {
    max_iterations: u32,
    step_size: f64,
    goal_bias: f64,
}

impl RRTPlanner {
    pub fn new(max_iterations: u32, step_size: f64, goal_bias: f64) -> Self {
        Self {
            max_iterations,
            step_size,
            goal_bias,
        }
    }

    pub async fn plan_path(
        &self,
        start: &RobotState,
        goal: &RobotState,
        map: &super::slam::OccupancyGrid,
    ) -> Result<Vec<RobotState>, String> {
        let mut tree = RRTree::new(start.clone());

        for _ in 0..self.max_iterations {
            let random_sample = if rand::random::<f64>() < self.goal_bias {
                goal.clone()
            } else {
                self.random_sample(map)
            };

            if let Some((nearest, _)) = tree.find_nearest(&random_sample) {
                let new_node = self.steer(nearest, &random_sample, map);

                if self.is_collision_free(nearest, &new_node, map) {
                    tree.add_node(new_node.clone(), nearest.clone());

                    if new_node.distance_to(goal) < self.step_size {
                        tree.add_node(goal.clone(), new_node);
                        return Ok(tree.get_path_to(goal));
                    }
                }
            }
        }

        Err("RRT failed to find path".to_string())
    }

    pub async fn plan_path_star(
        &self,
        start: &RobotState,
        goal: &RobotState,
        map: &super::slam::OccupancyGrid,
    ) -> Result<Vec<RobotState>, String> {
        // RRT* implementation would go here
        // For now, use basic RRT
        self.plan_path(start, goal, map).await
    }

    pub async fn plan_exploration(
        &self,
        start: &RobotState,
        frontier: &[RobotState],
        map: &super::slam::OccupancyGrid,
    ) -> Result<RobotState, String> {
        if frontier.is_empty() {
            return Err("No frontier points available".to_string());
        }

        // Elegir el punto frontera más cercano
        let target = frontier
            .iter()
            .min_by(|a, b| {
                start
                    .distance_to(a)
                    .partial_cmp(&start.distance_to(b))
                    .unwrap()
            })
            .unwrap();

        Ok(target.clone())
    }

    fn random_sample(&self, map: &super::slam::OccupancyGrid) -> RobotState {
        // Muestrear en el área conocida del mapa
        let bounds = map.get_known_bounds();

        RobotState::new(
            rand::random::<f64>() * bounds.width + bounds.min_x,
            rand::random::<f64>() * bounds.height + bounds.min_y,
            rand::random::<f64>() * 2.0 * std::f64::consts::PI,
        )
    }

    fn steer(
        &self,
        from: &RobotState,
        to: &RobotState,
        _map: &super::slam::OccupancyGrid,
    ) -> RobotState {
        let direction_x = to.x - from.x;
        let direction_y = to.y - from.y;
        let distance = (direction_x * direction_x + direction_y * direction_y).sqrt();

        if distance <= self.step_size {
            to.clone()
        } else {
            let scale = self.step_size / distance;
            RobotState::new(
                from.x + direction_x * scale,
                from.y + direction_y * scale,
                to.theta, // Mantener la orientación objetivo
            )
        }
    }

    fn is_collision_free(
        &self,
        from: &RobotState,
        to: &RobotState,
        map: &super::slam::OccupancyGrid,
    ) -> bool {
        // Verificar colisión a lo largo del segmento
        let steps = (from.distance_to(to) / 0.05).ceil() as usize; // 5cm resolution
        if steps == 0 {
            return true;
        }

        for i in 0..=steps {
            let t = i as f64 / steps as f64;
            let x = from.x + t * (to.x - from.x);
            let y = from.y + t * (to.y - from.y);

            if map.is_occupied(x, y, 0.2) {
                // 20cm safety margin
                return false;
            }
        }

        true
    }
}

// Estructuras auxiliares
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct GridNode {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct HeapNode {
    node: GridNode,
    f_score: f64,
}

impl PartialEq for HeapNode {
    fn eq(&self, other: &Self) -> bool {
        self.f_score == other.f_score
    }
}

impl Eq for HeapNode {}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.f_score.partial_cmp(&self.f_score) // Min-heap
    }
}

impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.f_score.partial_cmp(&self.f_score).unwrap()
    }
}

#[derive(Debug, Clone)]
struct RRTree {
    nodes: HashMap<RobotState, RobotState>, // node -> parent
}

impl RRTree {
    fn new(root: RobotState) -> Self {
        let mut nodes = HashMap::new();
        nodes.insert(root.clone(), root); // root points to itself
        Self { nodes }
    }

    fn add_node(&mut self, node: RobotState, parent: RobotState) {
        self.nodes.insert(node, parent);
    }

    fn find_nearest(&self, target: &RobotState) -> Option<(RobotState, f64)> {
        self.nodes
            .keys()
            .map(|node| (node.clone(), node.distance_to(target)))
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
    }

    fn get_path_to(&self, goal: &RobotState) -> Vec<RobotState> {
        let mut path = Vec::new();
        let mut current = goal;

        while let Some(parent) = self.nodes.get(current) {
            path.push(current.clone());
            if current == parent {
                break; // Reached root
            }
            current = parent;
        }

        path.reverse();
        path
    }
}
