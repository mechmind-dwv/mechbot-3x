use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RobotState {
    pub x: f64,                // Position x [m]
    pub y: f64,                // Position y [m]
    pub theta: f64,            // Orientation [rad]
    pub linear_velocity: f64,  // Linear velocity [m/s]
    pub angular_velocity: f64, // Angular velocity [rad/s]
    pub timestamp: f64,        // Timestamp [s]
}

impl Default for RobotState {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            theta: 0.0,
            linear_velocity: 0.0,
            angular_velocity: 0.0,
            timestamp: 0.0,
        }
    }
}

impl RobotState {
    pub fn new(x: f64, y: f64, theta: f64) -> Self {
        Self {
            x,
            y,
            theta,
            linear_velocity: 0.0,
            angular_velocity: 0.0,
            timestamp: 0.0,
        }
    }

    pub fn distance_to(&self, other: &RobotState) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    pub fn heading_to(&self, other: &RobotState) -> f64 {
        (other.y - self.y).atan2(other.x - self.x)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlInput {
    pub linear_x: f64,  // Linear velocity in x [m/s]
    pub linear_y: f64,  // Linear velocity in y [m/s] (for holonomic robots)
    pub angular_z: f64, // Angular velocity [rad/s]
}

impl Default for ControlInput {
    fn default() -> Self {
        Self {
            linear_x: 0.0,
            linear_y: 0.0,
            angular_z: 0.0,
        }
    }
}

impl ControlInput {
    pub fn new(linear_x: f64, angular_z: f64) -> Self {
        Self {
            linear_x,
            linear_y: 0.0,
            angular_z,
        }
    }

    pub fn zero() -> Self {
        Self::default()
    }

    pub fn magnitude(&self) -> f64 {
        (self.linear_x.powi(2) + self.linear_y.powi(2) + self.angular_z.powi(2)).sqrt()
    }
}

pub trait Controller {
    fn compute_control(
        &mut self,
        current_state: &RobotState,
        target_state: &RobotState,
        dt: f64,
    ) -> ControlInput;
    fn reset(&mut self);
    fn get_name(&self) -> String;
}

// Implementación básica de Controller para PID
impl Controller for super::pid::PIDController {
    fn compute_control(
        &mut self,
        current_state: &RobotState,
        target_state: &RobotState,
        dt: f64,
    ) -> ControlInput {
        // Simplified implementation - in practice you'd compute proper errors
        let distance_error = current_state.distance_to(target_state);
        let control_signal = self.compute(distance_error, dt);

        ControlInput::new(control_signal, 0.0)
    }

    fn reset(&mut self) {
        self.reset();
    }

    fn get_name(&self) -> String {
        "PID Controller".to_string()
    }
}
