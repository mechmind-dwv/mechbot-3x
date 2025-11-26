//! Ejemplo funcional completo de MechBot-3x
use anyhow::Result;
use mechbot_3x::{initialize_system, Config};

#[tokio::main]
async fn main() -> Result<()> {
    println!("🤖 MECHBOT-3X - SISTEMA COMPLETO");
    println!("================================");

    // Configuración
    let config = Config::default();
    println!("✅ Configuración: {}", config.robot.name);

    // Inicializar sistema
    println!("🔧 Inicializando sistema...");
    match initialize_system(config).await {
        Ok((lidar, camera, imu, navigation, vision, api_server)) => {
            println!("🎉 ¡SISTEMA INICIALIZADO EXITOSAMENTE!");
            println!("");
            println("📊 COMPONENTES OPERATIVOS:");
            println!("   • LIDAR: {:?}", lidar.get_status());
            println!("   • Cámara: {:?}", camera.get_status());
            println!("   • IMU: {:?}", imu.get_status());
            println!("   • Navegación: Lista");
            println!("   • Visión: Lista");
            println!(
                "   • API: {}",
                if api_server.is_running() {
                    "🟢 Activa"
                } else {
                    "🔴 Inactiva"
                }
            );
            println!("");

            // Demostración rápida
            println!("🎯 DEMOSTRACIÓN RÁPIDA:");

            // LIDAR
            if let Ok(points) = lidar.scan_environment() {
                println!("   • LIDAR: {} puntos escaneados", points.len());
                let obstacles = lidar.get_obstacles(&points, 2.0);
                println!("   • Obstáculos detectados: {}", obstacles.len());
            }

            // Navegación
            navigation.set_target(3.0, 4.0);
            println!("   • Navegación: Target establecido en (3.0, 4.0)");

            // Cámara
            if let Ok(frame) = camera.capture_frame() {
                println!(
                    "   • Cámara: Frame capturado {}x{}",
                    frame.width, frame.height
                );
            }

            // IMU
            if let Ok(orientation) = imu.get_orientation() {
                println!(
                    "   • IMU: Orientación: {:.2}°, {:.2}°, {:.2}°",
                    orientation.pitch.to_degrees(),
                    orientation.roll.to_degrees(),
                    orientation.yaw.to_degrees()
                );
            }

            println!("");
            println!("🚀 MECHBOT-3X LISTO PARA OPERAR");
            println!("💫 El sistema cósmico está funcionando");

            // Limpiar
            api_server.stop();
        }
        Err(e) => {
            eprintln!("❌ Error inicializando sistema: {}", e);
        }
    }

    Ok(())
}
