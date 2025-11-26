# Sistema de Control - MechBot-3x

## 🎯 Controladores Implementados

### PID Controller
```rust
pub struct PIDController {
    kp: f64,    // Proportional gain
    ki: f64,    // Integral gain  
    kd: f64,    // Derivative gain
    integral: f64,
    prev_error: f64,
}

impl PIDController {
    pub fn compute(&mut self, error: f64, dt: f64) -> f64 {
        self.integral += error * dt;
        let derivative = (error - self.prev_error) / dt;
        self.prev_error = error;
        
        self.kp * error + self.ki * self.integral + self.kd * derivative
    }
}
```

### Model Predictive Control (MPC)
```rust
pub struct MPCController {
    horizon: usize,
    constraints: Constraints,
    model: RobotModel,
}

impl MPCController {
    pub fn compute_control(&self, current_state: State, reference: &[State]) -> Result<ControlInput>;
}
```
