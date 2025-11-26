//! Ejemplo de movimiento básico con MechBot-3x
//!
//! Este ejemplo demuestra cómo controlar el movimiento básico del robot,
//! incluyendo navegación hacia waypoints y evitación de obstáculos.

use anyhow::Result;
use mechbot_3x::{initialize_system, Config, NavigationController};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 Iniciando ejemplo de movimiento básico...");

    // Configuración básica
    let config = Config::default();

    // Inicializar sistema
    let (mut lidar, _, _, mut navigation, _, mut api_server) = initialize_system(config).await?;

    println!("✅ Sistema inicializado");
    println!("🎯 Controlaremos el movimiento del robot paso a paso");

    // Definir waypoints para el recorrido
    let waypoints = vec![(2.0, 0.0), (2.0, 2.0), (0.0, 2.0), (0.0, 0.0)];

    // Recorrer cada waypoint
    for (i, &(target_x, target_y)) in waypoints.iter().enumerate() {
        println!(
            "\n📍 Navegando al waypoint {}: ({:.1}, {:.1})",
            i + 1,
            target_x,
            target_y
        );

        // Establecer target
        navigation.set_target(target_x, target_y);

        // Bucle de navegación hacia el waypoint
        let mut attempts = 0;
        while !navigation.is_at_target(0.1) && attempts < 50 {
            attempts += 1;

            // Escanear entorno en busca de obstáculos
            if let Ok(points) = lidar.scan_environment() {
                let obstacles = lidar.get_obstacles(&points, 1.5);

                // Recalcular ruta si hay obstáculos
                if !obstacles.is_empty() {
                    println!("⚠️  Obstáculos detectados: {}", obstacles.len());
                    navigation.calculate_path(
                        &obstacles.iter().map(|o| o.position).collect::<Vec<_>>(),
                    )?;
                }
            }

            // Simular movimiento (en implementación real, aquí se controlarían los motores)
            if let Some(next_waypoint) = navigation.get_next_waypoint() {
                println!(
                    "   → Moviendo a: ({:.2}, {:.2})",
                    next_waypoint.0, next_waypoint.1
                );
                navigation.update_position(next_waypoint.0, next_waypoint.1);
            }

            // Enviar telemetría
            api_server.send_telemetry(&format!(
                r#"{{"waypoint": {}, "position": {:?}, "attempt": {}}}"#,
                i + 1,
                navigation.current_position,
                attempts
            ))?;

            sleep(Duration::from_millis(100)).await;
        }

        if navigation.is_at_target(0.1) {
            println!("✅ Waypoint {} alcanzado!", i + 1);
        } else {
            println!("❌ No se pudo alcanzar el waypoint {}", i + 1);
        }
    }

    println!("\n🎉 Recorrido completado!");
    println!("📊 Posición final: {:?}", navigation.current_position);

    // Detener sistema
    api_server.stop();
    println!("🛑 Sistema detenido");

    Ok(())
}

/// Ejemplo adicional: movimiento en patrón circular
pub async fn circular_movement_example() -> Result<()> {
    println!("🌀 Iniciando movimiento circular...");

    let config = Config::default();
    let (_, _, _, mut navigation, _, _) = initialize_system(config).await?;

    let center = (5.0, 5.0);
    let radius = 3.0;
    let points = 36; // 10 grados por punto

    for i in 0..=points {
        let angle = (i as f64 / points as f64) * 2.0 * std::f64::consts::PI;
        let x = center.0 + radius * angle.cos();
        let y = center.1 + radius * angle.sin();

        navigation.set_target(x, y);
        navigation.update_position(x, y);

        println!("   ↻ Posición: ({:.2}, {:.2})", x, y);
        sleep(Duration::from_millis(50)).await;
    }

    println!("✅ Movimiento circular completado");
    Ok(())
}
