use anyhow::Result;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct GridPoint {
    x: i32,
    y: i32,
}

impl GridPoint {
    fn new(x: f64, y: f64) -> Self {
        Self {
            x: (x * 10.0).round() as i32, // Convertir a grid de 10cm
            y: (y * 10.0).round() as i32,
        }
    }

    fn to_world(self) -> (f64, f64) {
        (self.x as f64 / 10.0, self.y as f64 / 10.0)
    }
}

#[derive(Debug, Clone)]
struct Node {
    position: GridPoint,
    cost: f64,
    heuristic: f64,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        (self.cost + self.heuristic) == (other.cost + other.heuristic)
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // Orden inverso para min-heap
        other.cost.partial_cmp(&self.cost)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}

pub fn a_star(
    start: (f64, f64),
    goal: (f64, f64),
    obstacles: &[(f64, f64)],
) -> Result<Vec<(f64, f64)>> {
    let start_point = GridPoint::new(start.0, start.1);
    let goal_point = GridPoint::new(goal.0, goal.1);

    let mut open_set = BinaryHeap::new();
    let mut came_from = HashMap::new();
    let mut g_score = HashMap::new();

    g_score.insert(start_point, 0.0);
    open_set.push(Node {
        position: start_point,
        cost: 0.0,
        heuristic: heuristic(start_point, goal_point),
    });

    while let Some(current) = open_set.pop() {
        if current.position == goal_point {
            return Ok(reconstruct_path(came_from, current.position));
        }

        for neighbor in get_neighbors(current.position, obstacles) {
            let tentative_g_score =
                g_score[&current.position] + distance(current.position, neighbor);

            if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&f64::INFINITY) {
                came_from.insert(neighbor, current.position);
                g_score.insert(neighbor, tentative_g_score);

                open_set.push(Node {
                    position: neighbor,
                    cost: tentative_g_score,
                    heuristic: heuristic(neighbor, goal_point),
                });
            }
        }
    }

    anyhow::bail!(
        "No se pudo encontrar ruta del punto {:?} a {:?}",
        start,
        goal
    )
}

fn heuristic(a: GridPoint, b: GridPoint) -> f64 {
    // Distancia Manhattan para grid
    let dx = (a.x - b.x).abs() as f64;
    let dy = (a.y - b.y).abs() as f64;
    (dx + dy) / 10.0 // Convertir de vuelta a metros
}

fn distance(a: GridPoint, b: GridPoint) -> f64 {
    heuristic(a, b)
}

fn get_neighbors(position: GridPoint, obstacles: &[(f64, f64)]) -> Vec<GridPoint> {
    let directions = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];

    directions
        .iter()
        .map(|&(dx, dy)| GridPoint {
            x: position.x + dx,
            y: position.y + dy,
        })
        .filter(|&neighbor| !is_obstacle(neighbor.to_world(), obstacles))
        .collect()
}

fn is_obstacle(position: (f64, f64), obstacles: &[(f64, f64)]) -> bool {
    obstacles.iter().any(|&obs| {
        let dx = position.0 - obs.0;
        let dy = position.1 - obs.1;
        (dx * dx + dy * dy).sqrt() < 0.5 // Radio de colisión
    })
}

fn reconstruct_path(
    came_from: HashMap<GridPoint, GridPoint>,
    mut current: GridPoint,
) -> Vec<(f64, f64)> {
    let mut path = vec![current.to_world()];

    while let Some(&next) = came_from.get(&current) {
        path.push(next.to_world());
        current = next;
    }

    path.reverse();
    path
}
