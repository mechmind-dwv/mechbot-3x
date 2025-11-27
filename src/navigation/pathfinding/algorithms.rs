use super::{Path, GridMap};
use crate::navigation::Point;

pub enum PathfindingAlgorithm {
    AStar,
    Dijkstra,
    RRT,
}

pub fn find_path(
    start: Point,
    goal: Point,
    algorithm: PathfindingAlgorithm,
    map: &GridMap,
) -> Result<Path, String> {
    match algorithm {
        PathfindingAlgorithm::AStar => astar_path(start, goal, map),
        PathfindingAlgorithm::Dijkstra => dijkstra_path(start, goal, map),
        PathfindingAlgorithm::RRT => rrt_path(start, goal, map),
    }
}

fn astar_path(start: Point, goal: Point, map: &GridMap) -> Result<Path, String> {
    // Implementación de A*
    println!("Calculando ruta con A* de {:?} a {:?}", start, goal);
    Ok(Path {
        waypoints: vec![(start.x, start.y), (goal.x, goal.y)],
        length: 0.0,
    })
}

fn dijkstra_path(start: Point, goal: Point, map: &GridMap) -> Result<Path, String> {
    // Implementación de Dijkstra
    println!("Calculando ruta con Dijkstra");
    Ok(Path {
        waypoints: vec![(start.x, start.y), (goal.x, goal.y)],
        length: 0.0,
    })
}

fn rrt_path(start: Point, goal: Point, map: &GridMap) -> Result<Path, String> {
    // Implementación de RRT
    println!("Calculando ruta con RRT");
    Ok(Path {
        waypoints: vec![(start.x, start.y), (goal.x, goal.y)],
        length: 0.0,
    })
}
