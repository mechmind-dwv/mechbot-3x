pub mod pathfinding;
pub mod slam;

use anyhow::Result;

#[derive(Debug)]
pub struct NavigationController {
    pub current_position: (f64, f64),  // Público para facilitar
    target_position: (f64, f64),
    path: Vec<(f64, f64)>,
}

impl NavigationController {
    pub fn new() -> Self {
        Self {
            current_position: (0.0, 0.0),
            target_position: (0.0, 0.0),
            path: Vec::new(),
        }
    }

    pub fn set_target(&mut self, x: f64, y: f64) {
        self.target_position = (x, y);
        println!("🎯 Objetivo establecido: ({:.2}, {:.2})", x, y);
    }

    pub fn update_position(&mut self, x: f64, y: f64) {
        self.current_position = (x, y);
    }

    pub fn calculate_path(&mut self, obstacles: &[(f64, f64)]) -> Result<()> {
        self.path = pathfinding::a_star(
            self.current_position,
            self.target_position,
            obstacles
        )?;
        println!("🛣️  Ruta calculada con {} puntos", self.path.len());
        Ok(())
    }

    pub fn get_next_waypoint(&mut self) -> Option<(f64, f64)> {
        if self.path.is_empty() {
            None
        } else {
            Some(self.path.remove(0))
        }
    }

    pub fn is_at_target(&self, tolerance: f64) -> bool {
        let dx = self.current_position.0 - self.target_position.0;
        let dy = self.current_position.1 - self.target_position.1;
        (dx * dx + dy * dy).sqrt() <= tolerance
    }

    // Getters únicos
    pub fn get_current_position(&self) -> (f64, f64) {
        self.current_position
    }
    
    pub fn get_target_position(&self) -> (f64, f64) {
        self.target_position
    }
    
    pub fn get_current_path(&self) -> &Vec<(f64, f64)> {
        &self.path
    }
    
    pub fn set_current_position(&mut self, x: f64, y: f64) {
        self.current_position = (x, y);
    }
}

impl Default for NavigationController {
    fn default() -> Self {
        Self::new()
    }
}

pub use pathfinding::a_star;
pub use slam::SLAM;
