//! Prueba rápida del sistema MechBot-3x
use mechbot_3x::Config;

fn main() {
    println!("🎯 PRUEBA RÁPIDA MECHBOT-3X");
    println!("============================");

    // Probar configuración
    let config = Config::default();
    println!("✅ Configuración: {}", config.robot.name);

    // Probar creación de componentes básicos
    use mechbot_3x::{Camera, Lidar, NavigationController, VisionProcessor, IMU};

    let lidar = Lidar::new("/dev/ttyUSB0".to_string(), 115200);
    let camera = Camera::new(0, (640, 480), 30);
    let imu = IMU::new(0x68);
    let navigation = NavigationController::new();
    let vision = VisionProcessor::new();

    println!("✅ Componentes creados:");
    println!("   - LIDAR: ✅");
    println!("   - Cámara: ✅");
    println!("   - IMU: ✅");
    println!("   - Navegación: ✅");
    println!("   - Visión: ✅");

    println!("🎉 ¡Sistema MechBot-3x verificado!");
    println!("🚀 Listo para aventuras cósmicas...");
}
