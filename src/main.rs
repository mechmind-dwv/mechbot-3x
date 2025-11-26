use anyhow::Result;

mod config;
mod robot;

use crate::config::Config;
use crate::robot::Robot;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 MechBot-3x inicializado correctamente!");

    // Cargar configuración
    let config = Config::from_file("config.toml").unwrap_or_default();
    println!("✅ Configuración cargada: {}", config.robot.name);

    // Inicializar robot
    let robot = Robot::new(config).await?;
    println!("✅ Robot inicializado correctamente");

    println!("🎉 ¡MechBot-3x listo para operar!");
    Ok(())
}
