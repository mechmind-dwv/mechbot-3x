use mechbot_3x::{Robot, Config, sensors::*};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Inicializar robot con configuración
    let config = Config::from_file("config.toml")?;
    let mut robot = Robot::new(config).await?;
    
    // Iniciar sistemas
    robot.start_sensors().await?;
    robot.start_navigation().await?;
    
    // Comando de movimiento
    robot.move_to(100.0, 200.0).await?;
    
    // Modo autónomo
    robot.enable_autonomous_mode().await?;
    
    Ok(())
}
