use crate::control::base::{ControlInput, RobotState};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constraints {
    pub max_linear_velocity: f64,
    pub max_angular_velocity: f64,
    pub max_linear_acceleration: f64,
    pub max_angular_acceleration: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RobotModel {
    pub wheel_base: f64,
    pub wheel_radius: f64,
    pub max_steering_angle: f64,
}

impl Default for RobotModel {
    fn default() -> Self {
        Self {
            wheel_base: 0.5,
            wheel_radius: 0.1,
            max_steering_angle: std::f64::consts::PI / 4.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MPCController {
    horizon: usize,
    constraints: Constraints,
    model: RobotModel,
}

impl MPCController {
    pub fn new(horizon: usize) -> Self {
        Self {
            horizon,
            constraints: Constraints {
                max_linear_velocity: 1.0,
                max_angular_velocity: 3.0,
                max_linear_acceleration: 2.0,
                max_angular_acceleration: 5.0,
            },
            model: RobotModel::default(),
        }
    }

    pub fn compute_control(
        &self,
        current_state: RobotState,
        reference_trajectory: &[RobotState],
    ) -> Result<ControlInput, String> {
        if reference_trajectory.is_empty() {
            return Err("Reference trajectory is empty".to_string());
        }

        // Simplified MPC implementation
        // In a real implementation, this would solve an optimization problem

        let target_state = if reference_trajectory.len() > self.horizon {
            &reference_trajectory[self.horizon]
        } else {
            &reference_trajectory[reference_trajectory.len() - 1]
        };

        // Calculate errors
        let dx = target_state.x - current_state.x;
        let dy = target_state.y - current_state.y;
        let distance_error = (dx * dx + dy * dy).sqrt();

        // Calculate orientation error (normalized)
        let mut dtheta = target_state.theta - current_state.theta;
        while dtheta > std::f64::consts::PI {
            dtheta -= 2.0 * std::f64::consts::PI;
        }
        while dtheta < -std::f64::consts::PI {
            dtheta += 2.0 * std::f64::consts::PI;
        }

        // Simple control law for demonstration
        // In practice, this would be replaced with a proper optimization
        let linear_velocity = if distance_error > 0.1 {
            (0.5 * distance_error).min(self.constraints.max_linear_velocity)
        } else {
            0.0
        };

        let angular_velocity = if distance_error > 0.05 {
            (2.0 * dtheta).clamp(
                -self.constraints.max_angular_velocity,
                self.constraints.max_angular_velocity,
            )
        } else {
            0.0
        };

        Ok(ControlInput {
            linear_x: linear_velocity,
            linear_y: 0.0,
            angular_z: angular_velocity,
        })
    }

    pub fn set_constraints(&mut self, constraints: Constraints) {
        self.constraints = constraints;
    }

    pub fn set_model(&mut self, model: RobotModel) {
        self.model = model;
    }

    pub fn get_horizon(&self) -> usize {
        self.horizon
    }

    fn predict_trajectory(
        &self,
        initial_state: &RobotState,
        control_sequence: &[ControlInput],
        dt: f64,
    ) -> Vec<RobotState> {
        let mut trajectory = Vec::with_capacity(control_sequence.len() + 1);
        trajectory.push(initial_state.clone());

        let mut current_state = initial_state.clone();
        for control in control_sequence {
            // Simple unicycle model prediction
            let next_state = RobotState {
                x: current_state.x + control.linear_x * current_state.theta.cos() * dt,
                y: current_state.y + control.linear_x * current_state.theta.sin() * dt,
                theta: current_state.theta + control.angular_z * dt,
                linear_velocity: control.linear_x,
                angular_velocity: control.angular_z,
                timestamp: current_state.timestamp + dt,
            };
            trajectory.push(next_state.clone());
            current_state = next_state;
        }

        trajectory
    }

    fn cost_function(
        &self,
        predicted_trajectory: &[RobotState],
        reference_trajectory: &[RobotState],
        control_sequence: &[ControlInput],
    ) -> f64 {
        let mut cost = 0.0;

        // Tracking error cost
        for (i, (pred, ref_state)) in predicted_trajectory
            .iter()
            .zip(reference_trajectory.iter())
            .enumerate()
        {
            let position_error = (pred.x - ref_state.x).powi(2) + (pred.y - ref_state.y).powi(2);
            let orientation_error = (pred.theta - ref_state.theta).powi(2);
            cost += position_error + 0.1 * orientation_error;

            // Terminal cost (heavier weight on final state)
            if i == predicted_trajectory.len() - 1 {
                cost += 10.0 * (position_error + orientation_error);
            }
        }

        // Control effort cost
        for control in control_sequence {
            cost += 0.01 * (control.linear_x.powi(2) + control.angular_z.powi(2));
        }

        cost
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mpc_controller() {
        let mpc = MPCController::new(10);

        let current_state = RobotState {
            x: 0.0,
            y: 0.0,
            theta: 0.0,
            linear_velocity: 0.0,
            angular_velocity: 0.0,
            timestamp: 0.0,
        };

        let reference = vec![RobotState {
            x: 1.0,
            y: 0.0,
            theta: 0.0,
            linear_velocity: 0.0,
            angular_velocity: 0.0,
            timestamp: 0.0,
        }];

        let result = mpc.compute_control(current_state, &reference);
        assert!(result.is_ok());

        let control = result.unwrap();
        assert!(control.linear_x >= 0.0);
        assert!(control.linear_x <= mpc.constraints.max_linear_velocity);
    }
}
