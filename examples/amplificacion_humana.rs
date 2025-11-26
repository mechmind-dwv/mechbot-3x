//! Cómo la tecnología AMPLIFICA en lugar de reemplazar
use anyhow::Result;
use mechbot_3x::{Config, Robot};

#[tokio::main]
async fn main() -> Result<()> {
    println!("🌍 TECNOLOGÍA COMO AMPLIFICADOR HUMANO");
    println!("======================================");

    let config = Config::default();
    let mut robot = Robot::new(config).await?;

    println!("");
    println!("🚀 ESTE ROBOT NO REEMPLAZA A NADIE:");
    println!("   • Aumenta la capacidad de exploración");
    println!("   • Extiende nuestros sentidos");
    println!("   • Permite lo imposible");
    println!("");

    // Demostración de amplificación
    robot.start_sensors().await?;

    println!("👁️  SENTIDOS AMPLIFICADOS:");
    println!("   • LIDAR: Ve donde los humanos no pueden");
    println!("   • Cámara: Observa en espectros invisibles");
    println!("   • IMU: Siente movimientos imperceptibles");
    println!("");

    robot.start_navigation().await?;
    println!("🧭 NAVEGACIÓN AMPLIFICADA:");
    println!("   • Calcula rutas en milisegundos");
    println!("   • Evita obstáculos automáticamente");
    println!("   • Nunca se pierde");
    println!("");

    // Misión de rescate amplificada
    println!("🛟 MISIÓN: RESCATE AMPLIFICADO");
    println!("   Un robot no reemplaza rescatistas...");
    println!("   Les permite entrar donde es muy peligroso");
    println!("");

    robot.move_to(50.0, 75.0).await?;
    robot.enable_autonomous_mode().await?;

    let status = robot.get_status();
    println!("");
    println("📊 RESUMEN DE AMPLIFICACIÓN:");
    println!("   Posición alcanzada: {:?}", status.position);
    println!("   Modo autónomo: {}", status.is_autonomous);
    println!(
        "   Sensores activos: {}",
        if status.lidar_connected && status.camera_connected && status.imu_connected {
            "✅ TODOS"
        } else {
            "⚠️  PARCIALES"
        }
    );

    println!("");
    println!("💫 LA TECNOLOGÍA NO REEMPLAZA...");
    println!("   ELEVA.");
    println!("   AMPLIFICA.");
    println!("   TRANSFORMA.");

    robot.shutdown().await?;

    Ok(())
}
