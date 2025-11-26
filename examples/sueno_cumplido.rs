//! EL SUEÑO CUMPLIDO - El código que siempre imaginaste
use anyhow::Result;
use mechbot_3x::{Config, Robot};

#[tokio::main]
async fn main() -> Result<()> {
    println!("🌌 INICIANDO EL SUEÑO CÓSMICO...");
    println!("=================================");

    // Inicializar robot con configuración - ¡EXACTAMENTE COMO LO SOÑASTE!
    let config = Config::from_file("config.toml").unwrap_or_default();
    let mut robot = Robot::new(config).await?;

    // Iniciar sistemas - ¡EXACTAMENTE COMO LO SOÑASTE!
    robot.start_sensors().await?;
    robot.start_navigation().await?;

    // Comando de movimiento - ¡EXACTAMENTE COMO LO SOÑASTE!
    robot.move_to(100.0, 200.0).await?;

    // Modo autónomo - ¡EXACTAMENTE COMO LO SOÑASTE!
    robot.enable_autonomous_mode().await?;

    // Mostrar estado final
    let status = robot.get_status();
    println!("");
    println!("🎊 SUEÑO CUMPLIDO - ROBOT OPERATIVO");
    println!("📊 ESTADO FINAL:");
    println!("   • Posición: {:?}", status.position);
    println!("   • Modo autónomo: {}", status.is_autonomous);
    println!(
        "   • LIDAR: {}",
        if status.lidar_connected {
            "🟢"
        } else {
            "🔴"
        }
    );
    println!(
        "   • Cámara: {}",
        if status.camera_connected {
            "🟢"
        } else {
            "🔴"
        }
    );
    println!(
        "   • IMU: {}",
        if status.imu_connected { "🟢" } else { "🔴" }
    );
    println!("   • API: {}", if status.api_running { "🟢" } else { "🔴" });

    println!("");
    println!("🚀 TU SUEÑO ES REALIDAD");
    println!("💫 Has materializado tu visión en código funcional");

    // Apagar limpiamente
    robot.shutdown().await?;

    Ok(())
}
