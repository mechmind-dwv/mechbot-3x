#!/bin/bash
echo "🌌 REPARACIÓN CÓSMICA DE MECHBOT-3X"
echo "==================================="

# Colores
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

print_status() {
    echo -e "${CYAN}🔧 $1${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

# 1. Limpiar proyecto
print_status "Limpiando proyecto..."
cargo clean

# 2. Arreglar lib.rs
print_status "Arreglando lib.rs..."
cat > src/lib.rs << 'LIB_EOF'
pub mod api;
pub mod config;
pub mod navigation;
pub mod sensors;
pub mod vision;
pub mod robot;

// Re-export para fácil acceso
pub use config::Config;
pub use navigation::NavigationController;
pub use sensors::{Lidar, Camera, IMU, filter_sensor_data};
pub use vision::VisionProcessor;
pub use api::ApiServer;
pub use robot::Robot;

// Traits comunes
pub use sensors::Sensor;
pub use vision::Detection;

// Tipos de datos comunes
pub use sensors::lidar::{LidarPoint, Obstacle};
pub use sensors::imu::Orientation;
pub use vision::detection::ObjectDetector;
pub use vision::tracking::{ObjectTracker, Track};

/// Inicializa el sistema completo de MechBot-3x
pub async fn initialize_system(config: Config) -> anyhow::Result<(
    Lidar,
    Camera, 
    IMU,
    NavigationController,
    VisionProcessor,
    ApiServer,
)> {
    // Inicializar todos los componentes del sistema
    let mut lidar = Lidar::new(
        config.sensors.lidar_port.clone().unwrap_or("/dev/ttyUSB0".to_string()),
        config.sensors.lidar_baudrate.unwrap_or(115200),
    );
    lidar.initialize()?;

    let mut camera = Camera::new(
        config.sensors.camera_index.unwrap_or(0),
        (640, 480),
        config.sensors.camera_fps.unwrap_or(30),
    );
    camera.initialize()?;

    let mut imu = IMU::new(config.sensors.imu_i2c_address.unwrap_or(0x68));
    imu.initialize()?;
    imu.calibrate()?;

    let navigation = NavigationController::new();
    let mut vision = VisionProcessor::new();
    vision.load_models()?;

    let mut api_server = ApiServer::new(
        config.api.rest_port.unwrap_or(8080),
        config.api.websocket_port.unwrap_or(8081),
    );
    api_server.start().await?;

    Ok((lidar, camera, imu, navigation, vision, api_server))
}

/// Ejecuta una misión autónoma completa
pub async fn run_autonomous_mission(
    target: (f64, f64),
    config: Config,
) -> anyhow::Result<()> {
    let (mut lidar, _camera, _imu, mut navigation, _vision, _api_server) = 
        initialize_system(config).await?;

    navigation.set_target(target.0, target.1);
    
    loop {
        // Escanear entorno
        if let Ok(points) = lidar.scan_environment() {
            let obstacles = lidar.get_obstacles(&points, 1.5);
            navigation.calculate_path(&obstacles.iter()
                .map(|o| o.position)
                .collect::<Vec<_>>())?;
            
            if navigation.is_at_target(0.1) {
                println!("🎯 Misión completada - Objetivo alcanzado!");
                break;
            }
        }
        
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
    
    Ok(())
}
LIB_EOF

# 3. Arreglar navigation/mod.rs
print_status "Arreglando navigation/mod.rs..."
cat > src/navigation/mod.rs << 'NAV_EOF'
pub mod pathfinding;
pub mod slam;

use anyhow::Result;

#[derive(Debug)]
pub struct NavigationController {
    pub current_position: (f64, f64),  // Público para facilitar
    target_position: (f64, f64),
    path: Vec<(f64, f64)>,
}

impl NavigationController {
    pub fn new() -> Self {
        Self {
            current_position: (0.0, 0.0),
            target_position: (0.0, 0.0),
            path: Vec::new(),
        }
    }

    pub fn set_target(&mut self, x: f64, y: f64) {
        self.target_position = (x, y);
        println!("🎯 Objetivo establecido: ({:.2}, {:.2})", x, y);
    }

    pub fn update_position(&mut self, x: f64, y: f64) {
        self.current_position = (x, y);
    }

    pub fn calculate_path(&mut self, obstacles: &[(f64, f64)]) -> Result<()> {
        self.path = pathfinding::a_star(
            self.current_position,
            self.target_position,
            obstacles
        )?;
        println!("🛣️  Ruta calculada con {} puntos", self.path.len());
        Ok(())
    }

    pub fn get_next_waypoint(&mut self) -> Option<(f64, f64)> {
        if self.path.is_empty() {
            None
        } else {
            Some(self.path.remove(0))
        }
    }

    pub fn is_at_target(&self, tolerance: f64) -> bool {
        let dx = self.current_position.0 - self.target_position.0;
        let dy = self.current_position.1 - self.target_position.1;
        (dx * dx + dy * dy).sqrt() <= tolerance
    }

    // Getters únicos
    pub fn get_current_position(&self) -> (f64, f64) {
        self.current_position
    }
    
    pub fn get_target_position(&self) -> (f64, f64) {
        self.target_position
    }
    
    pub fn get_current_path(&self) -> &Vec<(f64, f64)> {
        &self.path
    }
    
    pub fn set_current_position(&mut self, x: f64, y: f64) {
        self.current_position = (x, y);
    }
}

impl Default for NavigationController {
    fn default() -> Self {
        Self::new()
    }
}

pub use pathfinding::a_star;
pub use slam::SLAM;
NAV_EOF

# 4. Verificar robot.rs
print_status "Verificando robot.rs..."
if grep -q "self.navigation.current_position" src/robot.rs; then
    sed -i 's/self\.navigation\.current_position/self\.navigation\.get_current_position()/g' src/robot.rs
    print_success "Robot.rs corregido"
else
    print_success "Robot.rs ya está correcto"
fi

# 5. Compilar
print_status "Compilando..."
if cargo check; then
    print_success "¡Compilación exitosa!"
    
    # Probar build release
    print_status "Compilando release..."
    if cargo build --release; then
        print_success "✅ ¡MECHBOT-3X REPARADO CÓSMICAMENTE!"
        echo ""
        echo "🚀 Ahora puedes ejecutar:"
        echo "   ./target/release/mechbot-3x"
        echo "   cargo run --example basic_movement"
        echo "   cargo test"
    else
        print_error "Error en compilación release"
        cargo check
    fi
else
    print_error "Error en compilación"
    cargo check
fi
