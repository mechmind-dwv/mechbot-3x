use crate::sensors::Sensor; // Importar el trait Sensor
use crate::{ApiServer, Camera, Config, Lidar, NavigationController, VisionProcessor, IMU};
use anyhow::Result;

/// El Robot unificado que siempre soñaste
pub struct Robot {
    lidar: Lidar,
    camera: Camera,
    imu: IMU,
    navigation: NavigationController,
    vision: VisionProcessor,
    api_server: ApiServer,
    config: Config,
    is_autonomous: bool,
}

impl Robot {
    /// Crea un nuevo robot con la configuración especificada
    pub async fn new(config: Config) -> Result<Self> {
        let (lidar, camera, imu, navigation, vision, api_server) =
            crate::initialize_system(config.clone()).await?;

        Ok(Self {
            lidar,
            camera,
            imu,
            navigation,
            vision,
            api_server,
            config,
            is_autonomous: false,
        })
    }

    /// Inicia todos los sensores del robot
    pub async fn start_sensors(&mut self) -> Result<()> {
        println!("🔧 Iniciando sensores...");
        self.lidar.initialize()?;
        self.camera.initialize()?;
        self.imu.initialize()?;
        println!("✅ Todos los sensores iniciados");
        Ok(())
    }

    /// Inicia el sistema de navegación
    pub async fn start_navigation(&mut self) -> Result<()> {
        println!("🧭 Iniciando navegación...");
        // El navigation controller ya está listo al crearse
        println!("✅ Sistema de navegación listo");
        Ok(())
    }

    /// Mueve el robot a una posición específica
    pub async fn move_to(&mut self, x: f64, y: f64) -> Result<()> {
        println!("🎯 Moviendo robot a posición: ({}, {})", x, y);
        self.navigation.set_target(x, y);

        // Simular el movimiento
        for i in 0..5 {
            println!("   → Progreso: {}/5", i + 1);
            if let Ok(points) = self.lidar.scan_environment() {
                let obstacles: Vec<_> = self
                    .lidar
                    .get_obstacles(&points, 2.0)
                    .iter()
                    .map(|o| o.position)
                    .collect();

                let _ = self.navigation.calculate_path(&obstacles);
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        }

        println!("✅ Movimiento completado");
        Ok(())
    }

    /// Habilita el modo autónomo
    pub async fn enable_autonomous_mode(&mut self) -> Result<()> {
        println!("🤖 Activando modo autónomo...");
        self.is_autonomous = true;

        // En modo autónomo, el robot toma sus propias decisiones
        println!("✅ Modo autónomo activado - Robot tomando decisiones");
        Ok(())
    }

    /// Obtiene el estado actual del robot
    pub fn get_status(&self) -> RobotStatus {
        RobotStatus {
            position: self.navigation.get_current_position(),
            is_autonomous: self.is_autonomous,
            lidar_connected: self.lidar.get_status().connected,
            camera_connected: self.camera.get_status().connected,
            imu_connected: self.imu.get_status().connected,
            api_running: self.api_server.is_running(),
        }
    }

    /// Detiene todos los sistemas del robot
    pub async fn shutdown(&mut self) -> Result<()> {
        println!("🛑 Apagando sistemas del robot...");
        self.api_server.stop();
        self.is_autonomous = false;
        println!("✅ Robot apagado correctamente");
        Ok(())
    }
}

/// Estado del robot para monitoreo
#[derive(Debug, Clone)]
pub struct RobotStatus {
    pub position: (f64, f64),
    pub is_autonomous: bool,
    pub lidar_connected: bool,
    pub camera_connected: bool,
    pub imu_connected: bool,
    pub api_running: bool,
}

impl Default for RobotStatus {
    fn default() -> Self {
        Self {
            position: (0.0, 0.0),
            is_autonomous: false,
            lidar_connected: false,
            camera_connected: false,
            imu_connected: false,
            api_running: false,
        }
    }
}
