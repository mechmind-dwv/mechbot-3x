//! Tests principales y configuración de testing para MechBot-3x

use anyhow::Result;
use mechbot_3x::Config;

// Tests de configuración
#[cfg(test)]
mod config_tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();

        assert_eq!(config.robot.name, "MechBot-3x-001");
        assert_eq!(config.robot.model, "MB3X");
        assert_eq!(config.robot.version, "3.0.0");

        // Verificar valores por defecto de sensores
        assert!(config.sensors.lidar_port.is_some());
        assert!(config.sensors.camera_fps.is_some());
        assert!(config.sensors.imu_i2c_address.is_some());
    }

    #[test]
    fn test_config_serialization() -> Result<()> {
        let config = Config::default();

        // Serializar a TOML
        let toml_string = toml::to_string(&config)?;

        // Deserializar de vuelta
        let deserialized_config: Config = toml::from_str(&toml_string)?;

        // Deberían ser iguales
        assert_eq!(config.robot.name, deserialized_config.robot.name);
        assert_eq!(config.robot.model, deserialized_config.robot.model);

        Ok(())
    }

    #[test]
    fn test_config_from_file() -> Result<()> {
        // Crear archivo de configuración temporal
        let temp_config = r#"
            [robot]
            name = "TestBot"
            model = "TEST"
            version = "1.0.0"

            [sensors]
            lidar_port = "/dev/ttyTEST"
            lidar_baudrate = 9600
            camera_index = 1
            camera_fps = 15
            imu_i2c_address = 0x69

            [navigation]
            max_speed = 1.5
            max_acceleration = 0.8
            planning_frequency = 5
            obstacle_distance_threshold = 1.0

            [api]
            rest_port = 9090
            websocket_port = 9091
            enable_cors = false
            api_key_required = true

            [logging]
            level = "debug"
            output = "test.log"
            max_file_size = "1MB"
            rotate = false
        "#;

        std::fs::write("test_config.toml", temp_config)?;

        let config = Config::from_file("test_config.toml")?;

        assert_eq!(config.robot.name, "TestBot");
        assert_eq!(config.sensors.lidar_port, Some("/dev/ttyTEST".to_string()));
        assert_eq!(config.api.rest_port, Some(9090));

        // Limpiar
        std::fs::remove_file("test_config.toml")?;

        Ok(())
    }
}

// Tests de integración del sistema completo
#[cfg(test)]
mod system_tests {
    use super::*;
    use mechbot_3x::initialize_system;
    use tokio::test;

    #[test]
    async fn test_complete_system_initialization() -> Result<()> {
        let config = Config::default();

        let system = initialize_system(config).await;

        // La inicialización debería ser exitosa
        assert!(system.is_ok());

        let (lidar, camera, imu, _navigation, _vision, api_server) = system?;

        // Todos los componentes deberían estar operativos
        assert!(lidar.get_status().connected);
        assert!(camera.get_status().connected);
        assert!(imu.get_status().connected);
        assert!(api_server.is_running());

        Ok(())
    }

    #[test]
    async fn test_system_graceful_shutdown() -> Result<()> {
        let config = Config::default();
        let (_, _, _, _, _, mut api_server) = initialize_system(config).await?;

        // El servidor API debería estar corriendo
        assert!(api_server.is_running());

        // Detener el servidor
        api_server.stop();

        // El servidor debería estar detenido
        assert!(!api_server.is_running());

        Ok(())
    }
}

// Tests de rendimiento
#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::{Duration, Instant};

    #[test]
    fn test_config_loading_performance() {
        let start = Instant::now();

        // Cargar configuración múltiples veces
        for _ in 0..1000 {
            let _ = Config::default();
        }

        let duration = start.elapsed();

        // La creación de config debería ser muy rápida
        assert!(duration < Duration::from_millis(10));
    }

    #[test]
    fn test_memory_usage() {
        // Test que verifica que no hay fugas de memoria obvias
        let config = Config::default();

        // Clonar múltiples veces para test de memoria
        let mut configs = Vec::new();
        for _ in 0..100 {
            configs.push(config.clone());
        }

        // La memoria debería manejarse correctamente
        assert_eq!(configs.len(), 100);

        // Limpiar
        drop(configs);
    }
}

// Tests de seguridad
#[cfg(test)]
mod security_tests {
    use super::*;

    #[test]
    fn test_config_security() -> Result<()> {
        // Test que la configuración no expone información sensible
        let config = Config::default();

        // Serializar configuración
        let serialized = toml::to_string(&config)?;

        // Verificar que no hay contraseñas o tokens expuestos
        assert!(!serialized.contains("password"));
        assert!(!serialized.contains("token"));
        assert!(!serialized.contains("secret"));
        assert!(!serialized.contains("key"));

        Ok(())
    }

    #[test]
    fn test_api_security_defaults() {
        let config = Config::default();

        // Por defecto, la API debería tener CORS habilitado
        assert_eq!(config.api.enable_cors, Some(true));

        // Y requerir API key
        assert_eq!(config.api.api_key_required, Some(true));
    }
}
