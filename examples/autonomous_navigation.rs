//! Ejemplo de navegación autónoma avanzada
//!
//! Este ejemplo demomina capacidades autónomas completas incluyendo:
//! - Navegación con SLAM
//! - Evitación dinámica de obstáculos  
//! - Planificación de rutas complejas
//! - Fusión de datos de múltiples sensores

use anyhow::Result;
use mechbot_3x::{
    initialize_system,
    navigation::{NavigationController, SLAM},
    sensors::{filter_sensor_data, Lidar},
    vision::VisionProcessor,
    Config,
};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<()> {
    println!("🧠 Iniciando Navegación Autónoma Avanzada...");

    let config = Config::default();
    let (mut lidar, mut camera, mut imu, mut navigation, mut vision, mut api_server) =
        initialize_system(config).await?;

    println!("✅ Sistema autónomo inicializado");
    println!("🎯 Iniciando mapeo y navegación autónoma...");

    // Inicializar SLAM para mapeo del entorno
    let mut slam = SLAM::new(0.05, 200, 200); // Alta resolución (5cm)

    // Definir puntos de interés para exploración
    let exploration_targets = vec![
        (10.0, 0.0),  // Norte
        (10.0, 10.0), // Noreste
        (0.0, 10.0),  // Este
        (0.0, 0.0),   // Regreso al inicio
    ];

    let mut current_target_index = 0;
    let mut map_updates = 0;

    // Bucle principal de exploración autónoma
    while current_target_index < exploration_targets.len() {
        let target = exploration_targets[current_target_index];
        println!(
            "\n🎯 Navegando hacia target {}: {:?}",
            current_target_index + 1,
            target
        );

        navigation.set_target(target.0, target.1);

        let mut target_reached = false;
        let mut obstacle_detected = false;

        // Bucle de navegación hacia el target actual
        while !target_reached && !obstacle_detected {
            // ===== FASE DE PERCEPCIÓN =====
            let sensor_data = tokio::try_join!(
                async { lidar.scan_environment() },
                async { camera.capture_frame() },
                async { imu.get_orientation() },
            )?;

            let (lidar_points, camera_frame, imu_orientation) = sensor_data;

            // Procesar datos LIDAR para SLAM y obstáculos
            let obstacles: Vec<_> = lidar
                .get_obstacles(&lidar_points, 2.0)
                .iter()
                .map(|o| o.position)
                .collect();

            // Actualizar mapa SLAM
            slam.update_with_lidar(navigation.current_position, &obstacles)?;
            map_updates += 1;

            // Procesar visión para detección de objetos
            let vision_detections = vision.process_frame(
                &camera_frame.data,
                camera_frame.width,
                camera_frame.height,
            )?;

            // ===== FASE DE PLANIFICACIÓN =====
            if !obstacles.is_empty() {
                println!("⚠️  Detectados {} obstáculos", obstacles.len());

                // Replanificar ruta considerando obstáculos
                navigation.calculate_path(&obstacles)?;

                // Si hay muchos obstáculos, considerar cambiar de target
                if obstacles.len() > 5 {
                    println!("🚧 Muchos obstáculos, reconsiderando ruta...");
                    obstacle_detected = true;
                }
            }

            // ===== FASE DE EJECUCIÓN =====
            if let Some(next_waypoint) = navigation.get_next_waypoint() {
                // Simular movimiento al siguiente waypoint
                navigation.update_position(next_waypoint.0, next_waypoint.1);

                println!(
                    "   🧭 Posición: ({:.2}, {:.2}) - Waypoint: ({:.2}, {:.2})",
                    navigation.current_position.0,
                    navigation.current_position.1,
                    next_waypoint.0,
                    next_waypoint.1
                );

                // Enviar telemetría completa
                api_server.send_telemetry(&format!(
                    r#"{{
                        "position": {:?},
                        "target": {:?},
                        "obstacles": {},
                        "detections": {},
                        "orientation": {:?},
                        "map_updates": {}
                    }}"#,
                    navigation.current_position,
                    target,
                    obstacles.len(),
                    vision_detections.len(),
                    imu_orientation,
                    map_updates
                ))?;
            }

            // Verificar si se alcanzó el target
            target_reached = navigation.is_at_target(0.2);

            // Pequeña pausa para simular tiempo real
            sleep(Duration::from_millis(200)).await;
        }

        if target_reached {
            println!(
                "✅ Target {} alcanzado exitosamente!",
                current_target_index + 1
            );
            current_target_index += 1;
        } else if obstacle_detected {
            println!("🔄 Evitando área con muchos obstáculos, pasando al siguiente target");
            current_target_index += 1;
        }

        // Pequeña pausa entre targets
        sleep(Duration::from_secs(1)).await;
    }

    println!("\n🎉 Exploración autónoma completada!");
    println!("📊 Estadísticas:");
    println!(
        "   - Targets alcanzados: {}/{}",
        current_target_index,
        exploration_targets.len()
    );
    println!("   - Actualizaciones de mapa: {}", map_updates);
    println!(
        "   - Mapa final: {}x{} celdas",
        slam.get_occupancy_grid()[0].len(),
        slam.get_occupancy_grid().len()
    );

    // Guardar mapa (en implementación real)
    println!("💾 Mapa listo para ser guardado");

    api_server.stop();
    Ok(())
}

/// Ejemplo de comportamiento reactivo a emergencias
pub async fn emergency_avoidance_example() -> Result<()> {
    println!("🚨 Iniciando ejemplo de evitación de emergencia...");

    let config = Config::default();
    let (mut lidar, _, _, mut navigation, _, _) = initialize_system(config).await?;

    navigation.set_target(8.0, 8.0);

    // Simular detección repentina de obstáculo cercano
    for i in 0..100 {
        let points = lidar.scan_environment()?;
        let obstacles = lidar.get_obstacles(&points, 0.5); // Umbral muy cercano

        if !obstacles.is_empty() {
            println!("🚨 OBSTÁCULO MUY CERCANO! Ejecutando maniobra evasiva!");

            // Movimiento evasivo rápido
            navigation.update_position(
                navigation.current_position.0 - 1.0,
                navigation.current_position.1 - 1.0,
            );

            println!("   🏃‍♂️ Retrocediendo a posición de seguridad");
        } else if navigation.is_at_target(0.1) {
            println!("✅ Target alcanzado de manera segura");
            break;
        } else {
            // Navegación normal
            if let Some(waypoint) = navigation.get_next_waypoint() {
                navigation.update_position(waypoint.0, waypoint.1);
            }
        }

        sleep(Duration::from_millis(50)).await;

        if i % 20 == 0 {
            println!("   📍 Progreso: {}/100", i);
        }
    }

    Ok(())
}
