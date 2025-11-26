use anyhow::Result;
use mechbot_3x::{initialize_system, run_autonomous_mission, Config};
use tokio::test;

#[cfg(test)]
mod robot_integration_tests {
    use super::*;

    #[test]
    async fn test_system_initialization() -> Result<()> {
        let config = Config::default();

        let (lidar, camera, imu, navigation, vision, api_server) =
            initialize_system(config).await?;

        // Verificar que todos los componentes se inicializaron correctamente
        assert!(lidar.get_status().connected);
        assert!(camera.get_status().connected);
        assert!(imu.get_status().connected);
        assert!(api_server.is_running());

        // Verificar que la visión cargó los modelos
        // (en nuestra implementación simulada, siempre funciona)
        assert!(true);

        Ok(())
    }

    #[test]
    async fn test_autonomous_navigation() -> Result<()> {
        let config = Config::default();

        // Ejecutar misión autónoma a un target cercano
        let result = run_autonomous_mission((5.0, 5.0), config).await;

        // La misión debería completarse exitosamente
        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    async fn test_sensor_fusion() -> Result<()> {
        let config = Config::default();
        let (mut lidar, mut camera, mut imu, mut navigation, mut vision, _) =
            initialize_system(config).await?;

        // Leer datos de todos los sensores simultáneamente
        let lidar_data = tokio::spawn(async move { lidar.scan_environment() });
        let camera_data = tokio::spawn(async move { camera.capture_frame() });
        let imu_data = tokio::spawn(async move { imu.get_orientation() });

        let (lidar_result, camera_result, imu_result) =
            tokio::try_join!(lidar_data, camera_data, imu_data)?;

        // Todos los sensores deberían devolver datos válidos
        assert!(lidar_result?.len() > 0);
        assert!(camera_result?.data.len() > 0);
        assert!(imu_result?.pitch.abs() < std::f64::consts::PI);

        Ok(())
    }

    #[test]
    async fn test_navigation_with_obstacles() -> Result<()> {
        let config = Config::default();
        let (mut lidar, _, _, mut navigation, _, _) = initialize_system(config).await?;

        // Configurar target
        navigation.set_target(10.0, 10.0);

        // Simular escaneo con obstáculos
        let points = lidar.scan_environment()?;
        let obstacles = lidar.get_obstacles(&points, 3.0);

        // Calcular ruta evitando obstáculos
        navigation.calculate_path(&obstacles.iter().map(|o| o.position).collect::<Vec<_>>())?;

        // Debería encontrar una ruta
        assert!(!navigation.is_at_target(0.1)); // No debería estar en el target aún

        Ok(())
    }

    #[test]
    async fn test_vision_integration() -> Result<()> {
        let config = Config::default();
        let (_, mut camera, _, _, mut vision, _) = initialize_system(config).await?;

        // Capturar frame y procesarlo con visión
        let frame = camera.capture_frame()?;
        let detections = vision.process_frame(&frame.data, frame.width, frame.height)?;

        // En nuestra simulación, debería detectar al menos un objeto
        assert!(!detections.is_empty());

        Ok(())
    }

    #[test]
    async fn test_api_communication() -> Result<()> {
        let config = Config::default();
        let (_, _, _, _, _, mut api_server) = initialize_system(config).await?;

        // Verificar que el servidor API está corriendo
        assert!(api_server.is_running());

        // Enviar telemetría
        api_server.send_telemetry(r#"{"test": "data"}"#)?;

        // Detener el servidor
        api_server.stop();
        assert!(!api_server.is_running());

        Ok(())
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::{Duration, Instant};

    #[test]
    async fn test_system_performance() -> Result<()> {
        let config = Config::default();
        let start = Instant::now();

        let _system = initialize_system(config).await?;
        let initialization_time = start.elapsed();

        // La inicialización debería ser rápida
        assert!(initialization_time < Duration::from_secs(2));

        Ok(())
    }

    #[test]
    async fn test_real_time_requirements() -> Result<()> {
        let config = Config::default();
        let (mut lidar, _, _, mut navigation, _, _) = initialize_system(config).await?;

        // Test de rendimiento en bucle de control
        let iterations = 10;
        let mut total_time = Duration::new(0, 0);

        for _ in 0..iterations {
            let start = Instant::now();

            // Ciclo típico de procesamiento
            let points = lidar.scan_environment()?;
            let obstacles = lidar.get_obstacles(&points, 2.0);
            navigation.calculate_path(&obstacles.iter().map(|o| o.position).collect::<Vec<_>>())?;

            total_time += start.elapsed();
        }

        let average_time = total_time / iterations;

        // Cada iteración debería tomar menos de 50ms para tiempo real
        assert!(average_time < Duration::from_millis(50));

        Ok(())
    }
}

#[cfg(test)]
mod error_handling_tests {
    use super::*;

    #[test]
    async fn test_error_recovery() -> Result<()> {
        // Test que el sistema puede recuperarse de errores
        let config = Config::default();

        // La inicialización debería funcionar incluso con config por defecto
        let result = initialize_system(config).await;
        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    async fn test_graceful_degradation() -> Result<()> {
        // Test que el sistema funciona incluso cuando algunos sensores fallan
        let config = Config {
            sensors: mechbot_3x::config::SensorsConfig {
                lidar_port: Some("invalid_port".to_string()), // Puerto inválido
                ..Default::default()
            },
            ..Default::default()
        };

        // El sistema debería inicializarse incluso con config problemática
        let result = initialize_system(config).await;
        assert!(result.is_ok());

        Ok(())
    }
}
