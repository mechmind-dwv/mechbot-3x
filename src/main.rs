use anyhow::Result;
use log::{info, error};

mod config;
mod sensors;

use crate::config::Config;

#[tokio::main]
async fn main() -> Result<()> {
    // Inicializar logging
    env_logger::init();
    
    info!("🚀 Iniciando MechBot-3x...");

    // Cargar configuración
    let config = match Config::from_file("config.toml") {
        Ok(cfg) => {
            info!("✅ Configuración cargada desde config.toml");
            cfg
        }
        Err(e) => {
            info!("⚠️  Usando configuración por defecto: {}", e);
            Config::default()
        }
    };

    info!("🤖 Robot: {} v{}", config.robot.name, config.robot.version);
    info!("🌐 API en puerto: {}", config.api.rest_port);

    // Ejemplo de uso del procesador de sensores
    let sensor_readings = vec![25.5, 30.2, 45.8, 60.1];
    
    match sensors::process_sensor_data(&sensor_readings) {
        Ok(_) => info!("✅ Procesamiento de sensores exitoso"),
        Err(e) => error!("❌ Error procesando sensores: {}", e),
    }

    // Filtrar datos
    let filtered = sensors::filter_sensor_data(&sensor_readings, 50.0);
    info!("📊 Datos filtrados: {:?}", filtered);

    info!("🎯 MechBot-3x listo para operar");
    
    // Mantener el programa corriendo
    tokio::signal::ctrl_c().await?;
    info!("👋 Apagando MechBot-3x...");
    
    Ok(())
}
