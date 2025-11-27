use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PIDController {
    kp: f64,             // Proportional gain
    ki: f64,             // Integral gain
    kd: f64,             // Derivative gain
    integral: f64,       // Integral accumulator
    prev_error: f64,     // Previous error for derivative
    integral_limit: f64, // Anti-windup limit
    output_limit: f64,   // Output saturation limit
}

impl PIDController {
    pub fn new(kp: f64, ki: f64, kd: f64, integral_limit: f64, output_limit: f64) -> Self {
        Self {
            kp,
            ki,
            kd,
            integral: 0.0,
            prev_error: 0.0,
            integral_limit,
            output_limit,
        }
    }

    pub fn compute(&mut self, error: f64, dt: f64) -> f64 {
        // Proportional term
        let proportional = self.kp * error;

        // Integral term with anti-windup
        self.integral += error * dt;
        self.integral = self
            .integral
            .clamp(-self.integral_limit, self.integral_limit);
        let integral = self.ki * self.integral;

        // Derivative term (filtered)
        let derivative = if dt > 1e-6 {
            self.kd * (error - self.prev_error) / dt
        } else {
            0.0
        };
        self.prev_error = error;

        // Compute output and apply saturation
        let output = proportional + integral + derivative;
        output.clamp(-self.output_limit, self.output_limit)
    }

    pub fn reset(&mut self) {
        self.integral = 0.0;
        self.prev_error = 0.0;
    }

    pub fn set_gains(&mut self, kp: f64, ki: f64, kd: f64) {
        self.kp = kp;
        self.ki = ki;
        self.kd = kd;
    }

    pub fn get_gains(&self) -> (f64, f64, f64) {
        (self.kp, self.ki, self.kd)
    }

    pub fn get_integral(&self) -> f64 {
        self.integral
    }

    pub fn get_prev_error(&self) -> f64 {
        self.prev_error
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pid_controller() {
        let mut pid = PIDController::new(1.0, 0.1, 0.01, 10.0, 5.0);

        // Test step response
        let error = 1.0;
        let dt = 0.1;
        let output = pid.compute(error, dt);

        assert!(output > 0.0);
        assert!(output <= 5.0); // Should respect output limit
    }

    #[test]
    fn test_pid_reset() {
        let mut pid = PIDController::new(1.0, 0.1, 0.01, 10.0, 5.0);

        pid.compute(1.0, 0.1);
        assert!(pid.get_integral() > 0.0);

        pid.reset();
        assert_eq!(pid.get_integral(), 0.0);
        assert_eq!(pid.get_prev_error(), 0.0);
    }
}
