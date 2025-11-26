use mechbot_3x::Sensor;
use anyhow::Result;
use log::{info, warn};
use mechbot_3x::{initialize_system, Config};

#[tokio::main]
async fn main() -> Result<()> {
    // Inicializar logger
    env_logger::init();

    info!("🚀 MechBot-3x Inicializando Sistema Cósmico...");

    // Cargar configuración
    let config = match Config::from_file("config.toml") {
        Ok(cfg) => {
            info!("✅ Configuración cargada: {}", cfg.robot.name);
            cfg
        }
        Err(e) => {
            warn!(
                "⚠️  Error cargando config.toml: {}. Usando configuración por defecto.",
                e
            );
            Config::default()
        }
    };

    info!("🔧 Inicializando sistema completo...");

    match initialize_system(config).await {
        Ok((lidar, camera, imu, navigation, vision, mut api_server)) => {
            info!("✅ Todos los sistemas inicializados correctamente!");

            // Mostrar estado de los componentes
            info!("📊 Estado del sistema:");
            info!(
                "   - LIDAR: {}",
                if lidar.get_status().connected {
                    "🟢 Conectado"
                } else {
                    "🔴 Desconectado"
                }
            );
            info!(
                "   - Cámara: {}",
                if camera.get_status().connected {
                    "🟢 Conectada"
                } else {
                    "🔴 Desconectada"
                }
            );
            info!(
                "   - IMU: {}",
                if imu.get_status().connected {
                    "🟢 Conectado"
                } else {
                    "🔴 Desconectado"
                }
            );
            info!("   - Visión: {}", "🟢 Modelos cargados");
            info!(
                "   - API: {}",
                if api_server.is_running() {
                    "🟢 Servidor activo"
                } else {
                    "🔴 Servidor inactivo"
                }
            );
            info!("   - Navegación: 🟢 Controlador listo");

            // Ejecutar una misión de demostración simple
            info!("🎯 Iniciando misión de demostración...");

            // Simular un pequeño recorrido
            let demo_targets = [(2.0, 0.0), (2.0, 2.0), (0.0, 0.0)];

            for (i, &target) in demo_targets.iter().enumerate() {
                info!("📍 Navegando al punto {}: {:?}", i + 1, target);

                // En una implementación real aquí iría la lógica de navegación
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

                info!("✅ Punto {} alcanzado", i + 1);
            }

            info!("🎉 Misión de demostración completada!");

            // Mantener el sistema corriendo por un tiempo
            info!("⏰ Sistema operativo por 10 segundos...");
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

            api_server.stop();
            info!("🛑 Sistema apagado correctamente");
        }
        Err(e) => {
            eprintln!("❌ Error inicializando el sistema: {}", e);
            eprintln!("💡 Posibles soluciones:");
            eprintln!("   - Verificar que config.toml existe y es válido");
            eprintln!("   - Revisar dependencias con 'cargo check'");
            eprintln!("   - Ejecutar 'cargo clean && cargo build'");
            return Err(e);
        }
    }

    Ok(())
}
