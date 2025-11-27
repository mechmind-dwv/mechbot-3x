pub mod base;
pub mod mpc;
pub mod pid;

pub use base::{ControlInput, Controller, RobotState};
pub use mpc::MPCController;
pub use pid::PIDController;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlConfig {
    pub pid_linear: PIDConfig,
    pub pid_angular: PIDConfig,
    pub mpc: MPCConfig,
    pub max_linear_speed: f64,
    pub max_angular_speed: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PIDConfig {
    pub kp: f64,
    pub ki: f64,
    pub kd: f64,
    pub integral_limit: f64,
    pub output_limit: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MPCConfig {
    pub horizon: usize,
    pub dt: f64,
    pub max_iterations: usize,
    pub tolerance: f64,
}

impl Default for ControlConfig {
    fn default() -> Self {
        Self {
            pid_linear: PIDConfig {
                kp: 2.0,
                ki: 0.1,
                kd: 0.5,
                integral_limit: 1.0,
                output_limit: 1.0,
            },
            pid_angular: PIDConfig {
                kp: 3.0,
                ki: 0.05,
                kd: 0.3,
                integral_limit: 0.5,
                output_limit: 2.0,
            },
            mpc: MPCConfig {
                horizon: 10,
                dt: 0.1,
                max_iterations: 100,
                tolerance: 1e-4,
            },
            max_linear_speed: 1.0,
            max_angular_speed: 3.0,
        }
    }
}

pub struct ControlSystem {
    pub pid_linear: PIDController,
    pub pid_angular: PIDController,
    pub mpc: MPCController,
    pub config: ControlConfig,
}

impl ControlSystem {
    pub fn new(config: ControlConfig) -> Self {
        Self {
            pid_linear: PIDController::new(
                config.pid_linear.kp,
                config.pid_linear.ki,
                config.pid_linear.kd,
                config.pid_linear.integral_limit,
                config.pid_linear.output_limit,
            ),
            pid_angular: PIDController::new(
                config.pid_angular.kp,
                config.pid_angular.ki,
                config.pid_angular.kd,
                config.pid_angular.integral_limit,
                config.pid_angular.output_limit,
            ),
            mpc: MPCController::new(config.mpc.horizon),
            config,
        }
    }

    pub fn compute_pid_control(
        &mut self,
        current_pose: &RobotState,
        target_pose: &RobotState,
        dt: f64,
    ) -> ControlInput {
        // Error de posición
        let dx = target_pose.x - current_pose.x;
        let dy = target_pose.y - current_pose.y;

        // Error de orientación (normalizado a [-pi, pi])
        let mut dtheta = target_pose.theta - current_pose.theta;
        while dtheta > std::f64::consts::PI {
            dtheta -= 2.0 * std::f64::consts::PI;
        }
        while dtheta < -std::f64::consts::PI {
            dtheta += 2.0 * std::f64::consts::PI;
        }

        // Control de velocidad lineal (basado en distancia)
        let distance_error = (dx * dx + dy * dy).sqrt();
        let linear_velocity = self.pid_linear.compute(distance_error, dt);

        // Control de velocidad angular (basado en error de orientación)
        let angular_velocity = self.pid_angular.compute(dtheta, dt);

        ControlInput {
            linear_x: linear_velocity
                .clamp(-self.config.max_linear_speed, self.config.max_linear_speed),
            linear_y: 0.0,
            angular_z: angular_velocity.clamp(
                -self.config.max_angular_speed,
                self.config.max_angular_speed,
            ),
        }
    }

    pub fn compute_mpc_control(
        &self,
        current_state: &RobotState,
        reference_trajectory: &[RobotState],
    ) -> Result<ControlInput, String> {
        self.mpc
            .compute_control(current_state.clone(), reference_trajectory)
    }

    pub fn reset(&mut self) {
        self.pid_linear.reset();
        self.pid_angular.reset();
    }
}
