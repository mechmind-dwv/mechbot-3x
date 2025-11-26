//! Ejemplo mínimo del sistema MechBot-3x
use anyhow::Result;
use mechbot_3x::Config;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🎯 Iniciando ejemplo mínimo de MechBot-3x...");

    let config = Config::default();
    println!("✅ Configuración cargada: {}", config.robot.name);

    // Solo probar la creación de componentes básicos
    use mechbot_3x::navigation::NavigationController;
    use mechbot_3x::sensors::{Camera, Lidar, IMU};
    use mechbot_3x::vision::VisionProcessor;

    let lidar = Lidar::new("/dev/ttyUSB0".to_string(), 115200);
    let camera = Camera::new(0, (640, 480), 30);
    let imu = IMU::new(0x68);
    let navigation = NavigationController::new();
    let vision = VisionProcessor::new();

    println!("✅ Todos los componentes creados exitosamente!");
    println!("📊 Resumen:");
    println!("   - LIDAR: {:?}", lidar);
    println!("   - Cámara: {:?}", camera);
    println!("   - IMU: {:?}", imu);
    println!("   - Navegación: {:?}", navigation);
    println!("   - Visión: {:?}", vision);

    println!("🎉 ¡Ejemplo mínimo ejecutado con éxito!");
    Ok(())
}
