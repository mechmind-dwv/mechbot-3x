// Ejemplo de uso del sistema de control MechBot-3X
// Ejecutar con: cargo run --example control_system

use mechbot_3x::control::{ControlConfig, ControlSystem, RobotState};

fn main() {
    println!("🎯 Sistema de Control MechBot-3X");
    println!("=================================");

    // Crear configuración de control
    let config = ControlConfig::default();
    let mut control_system = ControlSystem::new(config);

    // Estado actual del robot
    let current_state = RobotState::new(0.0, 0.0, 0.0);

    // Estado objetivo
    let target_state = RobotState::new(2.0, 1.0, 0.0);

    println!(
        "📍 Estado actual: ({:.2}, {:.2}, {:.2})",
        current_state.x, current_state.y, current_state.theta
    );
    println(
        "🎯 Estado objetivo: ({:.2}, {:.2}, {:.2})",
        target_state.x,
        target_state.y,
        target_state.theta,
    );

    // Simular control loop
    let dt = 0.1; // 100ms
    let mut simulated_state = current_state.clone();

    for step in 0..50 {
        // Computar control PID
        let control_input = control_system.compute_pid_control(&simulated_state, &target_state, dt);

        // Simular dinámica del robot (modelo uniciclo simple)
        simulated_state.x += control_input.linear_x * simulated_state.theta.cos() * dt;
        simulated_state.y += control_input.linear_x * simulated_state.theta.sin() * dt;
        simulated_state.theta += control_input.angular_z * dt;

        let distance_to_target = simulated_state.distance_to(&target_state);

        println!(
            "Step {}: Pos=({:.2}, {:.2}), θ={:.2}, Control=({:.2}, {:.2}), Dist={:.3}",
            step,
            simulated_state.x,
            simulated_state.y,
            simulated_state.theta,
            control_input.linear_x,
            control_input.angular_z,
            distance_to_target
        );

        if distance_to_target < 0.05 {
            println!("✅ Objetivo alcanzado!");
            break;
        }
    }

    println!("🎉 Simulación de control completada");
}
