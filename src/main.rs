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

            // Mostrar características de visión
            let vision_features = vision.get_available_features();
            info!("   - Visión: 🟢 {} cargado", vision_features.join(", "));

            info!(
                "   - API: {}",
                if api_server.is_running() {
                    "🟢 Servidor activo"
                } else {
                    "🔴 Servidor inactivo"
                }
            );
            info!("   - Navegación: 🟢 Controlador listo");

            // Iniciar servidores API
            let api_handle = tokio::spawn(async move {
                if let Err(e) = api_server.start().await {
                    eprintln!("❌ Error en servidor API: {}", e);
                }
            });

            info!("🌐 Servidores API iniciados:");
            info!("   - REST:    http://localhost:{}", api_server.port);
            info!("   - WebSocket: ws://localhost:{}", api_server.port + 1);
            info!("");
            info!("📖 Endpoints disponibles:");
            info!(
                "   GET  http://localhost:{}/api/v1/status    - Estado del robot",
                api_server.port
            );
            info!(
                "   POST http://localhost:{}/api/v1/move      - Mover a posición",
                api_server.port
            );
            info!(
                "   GET  http://localhost:{}/api/v1/map       - Mapa actual",
                api_server.port
            );
            info!(
                "   GET  http://localhost:{}/api/v1/sensors   - Datos de sensores",
                api_server.port
            );
            info!(
                "   GET  http://localhost:{}/health           - Health check",
                api_server.port
            );
            info!("");
            info!(
                "🔌 WebSocket: ws://localhost:{}/telemetry",
                api_server.port + 1
            );

            // Ejecutar una misión de demostración simple
            info!("🎯 Iniciando misión de demostración...");

            // Simular un pequeño recorrido
            let demo_targets = [(2.0, 0.0), (2.0, 2.0), (0.0, 0.0)];

            for (i, &target) in demo_targets.iter().enumerate() {
                info!("📍 Navegando al punto {}: {:?}", i + 1, target);
                tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                info!("✅ Punto {} alcanzado", i + 1);
            }

            info!("🎉 Misión de demostración completada!");

            // Mantener el sistema corriendo
            info!("⏰ Sistema operativo. Presiona Ctrl+C para detener...");

            // Esperar a que el usuario detenga el sistema
            tokio::signal::ctrl_c().await?;

            info!("🛑 Deteniendo sistema...");
            api_server.stop();

            // Esperar a que el servidor API se detenga
            api_handle.await?;

            info!("🎊 Sistema apagado correctamente. ¡Hasta la próxima!");
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
