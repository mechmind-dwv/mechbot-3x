// Ejemplo de navegación y pathfinding MechBot-3X
// Ejecutar con: cargo run --example navigation_demo

use mechbot_3x::control::RobotState;
use mechbot_3x::navigation::{NavigationConfig, NavigationController, SensorData};

#[tokio::main]
async fn main() {
    println!("🧭 Sistema de Navegación MechBot-3X");
    println!("===================================");

    // Crear controlador de navegación
    let config = NavigationConfig::default();
    let mut nav_controller = NavigationController::new(config);

    // Estado inicial del robot
    let start_pose = RobotState::new(0.0, 0.0, 0.0);
    let goal_pose = RobotState::new(5.0, 3.0, 1.57); // 5m adelante, 3m a la derecha, 90 grados

    println!(
        "📍 Pose inicial: ({:.2}, {:.2}, {:.2})",
        start_pose.x, start_pose.y, start_pose.theta
    );
    println!(
        "🎯 Objetivo: ({:.2}, {:.2}, {:.2})",
        goal_pose.x, goal_pose.y, goal_pose.theta
    );

    // Simular datos de sensores (LIDAR)
    let sensor_data = SensorData {
        lidar_scan: vec![
            (2.0, 0.0),    // 2m adelante
            (1.5, 0.785),  // 1.5m a 45 grados
            (3.0, -0.785), // 3m a -45 grados
        ],
        odometry: (0.0, 0.0, 0.0), // Sin movimiento
        timestamp: 0.0,
    };

    // Ejecutar navegación
    match nav_controller
        .navigate_to_pose(goal_pose, start_pose, &sensor_data)
        .await
    {
        Ok(control_input) => {
            println!("✅ Comando de control generado:");
            println!("   - Velocidad lineal: {:.2} m/s", control_input.linear_x);
            println!(
                "   - Velocidad angular: {:.2} rad/s",
                control_input.angular_z
            );

            // Mostrar camino planificado
            if let Some(path) = nav_controller.get_current_path() {
                println!("🛣️  Camino planificado ({} puntos):", path.len());
                for (i, point) in path.iter().enumerate().take(5) {
                    println!(
                        "   {}. ({:.2}, {:.2}, {:.2})",
                        i, point.x, point.y, point.theta
                    );
                }
                if path.len() > 5 {
                    println!("   ... y {} puntos más", path.len() - 5);
                }
            }
        }
        Err(e) => {
            println!("❌ Error en navegación: {}", e);
        }
    }

    // Mostrar información del mapa SLAM
    let map = nav_controller.get_map();
    let bounds = map.get_known_bounds();
    println!("🗺️  Mapa SLAM:");
    println!(
        "   - Tamaño: {:.1} x {:.1} metros",
        bounds.width, bounds.height
    );
    println!("   - Resolución: {:.3} metros", map.resolution);

    println!("🎉 Demostración de navegación completada");
}
