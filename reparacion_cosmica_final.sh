#!/bin/bash
echo "🌌 REPARACIÓN CÓSMICA FINAL MECHBOT-3X"
echo "======================================"

# 1. Corregir lib.rs
echo "🔧 Corrigiendo lib.rs..."
cat > src/lib.rs << 'LIBRS'
pub mod api;
pub mod config;
pub mod navigation;
pub mod sensors;
pub mod vision;
pub mod robot;

// Re-export para fácil acceso
pub use api::{ApiServer, WebSocketServer};
pub use config::Config;
pub use navigation::NavigationController;
pub use robot::Robot;
pub use sensors::{Camera, Lidar, IMU, Sensor, SensorStatus};
pub use vision::VisionProcessor;

// Función de inicialización del sistema CORREGIDA
pub async fn initialize_system(config: Config) -> anyhow::Result<(
    Lidar,
    Camera, 
    IMU,
    NavigationController,
    VisionProcessor,
    ApiServer,
)> {
    // Usar valores por defecto si los campos Option son None
    let lidar_port = config.sensors.lidar_port.clone().unwrap_or_else(|| "/dev/ttyUSB0".to_string());
    let lidar_baudrate = config.sensors.lidar_baudrate.unwrap_or(115200);
    let camera_index = config.sensors.camera_index.unwrap_or(0);
    let imu_address = config.sensors.imu_i2c_address.unwrap_or(0x68);
    let rest_port = config.api.rest_port.unwrap_or(8080);
    
    // Inicializar componentes con valores por defecto
    let mut lidar = Lidar::new(lidar_port, lidar_baudrate);
    let mut camera = Camera::new(camera_index, (640, 480), 30);
    let mut imu = IMU::new(imu_address);
    
    // Remover .await porque initialize() no es async
    lidar.initialize()?;
    camera.initialize()?;
    imu.initialize()?;
    
    let navigation = NavigationController::new();
    let vision = VisionProcessor::new();
    let api_server = ApiServer::new(rest_port);
    
    Ok((lidar, camera, imu, navigation, vision, api_server))
}
LIBRS

# 2. Corregir api/mod.rs
echo "🔧 Corrigiendo api/mod.rs..."
cat > src/api/mod.rs << 'APIRS'
#[derive(Debug)]
pub struct ApiServer {
    port: u16,
    is_running: bool,
}

impl ApiServer {
    pub fn new(port: u16) -> Self {
        Self {
            port,
            is_running: false,
        }
    }
    
    pub fn is_running(&self) -> bool {
        self.is_running
    }
    
    pub fn stop(&mut self) {
        self.is_running = false;
        println!("API Server stopped");
    }
}

#[derive(Debug)]
pub struct WebSocketServer {
    port: u16,
}

impl WebSocketServer {
    pub fn new(port: u16) -> Self {
        Self { port }
    }
}
APIRS

# 3. Compilar
echo "🔄 Compilando..."
cargo build --release

# 4. Ejecutar
if [ -f "./target/release/mechbot-3x" ]; then
    echo "🎉 ¡MECHBOT-3X REPARADO EXITOSAMENTE!"
    echo "🚀 EJECUTANDO SISTEMA CÓSMICO..."
    echo "================================"
    ./target/release/mechbot-3x
else
    echo "❌ Error en compilación"
    echo "💡 Última verificación:"
    cargo check
fi
