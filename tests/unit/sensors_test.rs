use anyhow::Result;
use mechbot_3x::sensors::{filter_sensor_data, Camera, Lidar, Sensor, IMU};

#[cfg(test)]
mod sensor_tests {
    use super::*;

    #[test]
    fn test_lidar_initialization() -> Result<()> {
        let mut lidar = Lidar::new("/dev/ttyUSB0".to_string(), 115200);
        lidar.initialize()?;

        assert!(lidar.get_status().connected);
        Ok(())
    }

    #[test]
    fn test_lidar_scanning() -> Result<()> {
        let mut lidar = Lidar::new("/dev/ttyUSB0".to_string(), 115200);
        lidar.initialize()?;

        let points = lidar.scan_environment()?;
        assert_eq!(points.len(), 360); // 360 puntos por escaneo

        // Verificar que los puntos tienen valores válidos
        for point in points {
            assert!(point.distance >= 0.0);
            assert!(point.angle >= 0.0 && point.angle < 360.0);
            assert!(point.intensity >= 0.0 && point.intensity <= 1.0);
        }

        Ok(())
    }

    #[test]
    fn test_obstacle_detection() -> Result<()> {
        let mut lidar = Lidar::new("/dev/ttyUSB0".to_string(), 115200);
        lidar.initialize()?;

        let points = lidar.scan_environment()?;
        let obstacles = lidar.get_obstacles(&points, 2.0);

        // En nuestro entorno simulado, debería haber obstáculos
        assert!(!obstacles.is_empty());
        Ok(())
    }

    #[test]
    fn test_camera_initialization() -> Result<()> {
        let mut camera = Camera::new(0, (640, 480), 30);
        camera.initialize()?;

        assert!(camera.get_status().connected);
        Ok(())
    }

    #[test]
    fn test_camera_frame_capture() -> Result<()> {
        let mut camera = Camera::new(0, (640, 480), 30);
        camera.initialize()?;

        let frame = camera.capture_frame()?;
        assert_eq!(frame.width, 640);
        assert_eq!(frame.height, 480);
        assert_eq!(frame.data.len(), 640 * 480 * 3); // RGB

        Ok(())
    }

    #[test]
    fn test_imu_initialization() -> Result<()> {
        let mut imu = IMU::new(0x68);
        imu.initialize()?;

        assert!(imu.get_status().connected);
        Ok(())
    }

    #[test]
    fn test_imu_sensor_readings() -> Result<()> {
        let mut imu = IMU::new(0x68);
        imu.initialize()?;

        let (ax, ay, az) = imu.read_accelerometer()?;
        let (gx, gy, gz) = imu.read_gyroscope()?;
        let (mx, my, mz) = imu.read_magnetometer()?;

        // Verificar rangos razonables
        assert!(ax.abs() < 20.0 && ay.abs() < 20.0 && az.abs() < 20.0);
        assert!(gx.abs() < 10.0 && gy.abs() < 10.0 && gz.abs() < 10.0);
        assert!(mx.abs() < 100.0 && my.abs() < 100.0 && mz.abs() < 100.0);

        Ok(())
    }

    #[test]
    fn test_imu_orientation() -> Result<()> {
        let mut imu = IMU::new(0x68);
        imu.initialize()?;

        let orientation = imu.get_orientation()?;

        // Verificar que los ángulos de orientación están en radianes
        assert!(orientation.pitch.abs() < std::f64::consts::PI);
        assert!(orientation.roll.abs() < std::f64::consts::PI);
        assert!(orientation.yaw.abs() < std::f64::consts::PI * 2.0);

        Ok(())
    }

    #[test]
    fn test_sensor_data_filtering() {
        let test_data = vec![1.0, 2.0, 3.0, 4.0, 5.0];

        // Test moving average filter
        let filtered = filter_sensor_data(&test_data, "moving_average");
        assert_eq!(filtered.len(), 3); // (1+2+3)/3, (2+3+4)/3, (3+4+5)/3

        // Test Kalman filter
        let kalman_filtered = filter_sensor_data(&test_data, "kalman");
        assert_eq!(kalman_filtered.len(), test_data.len());

        // Test default (no filter)
        let default_filtered = filter_sensor_data(&test_data, "none");
        assert_eq!(default_filtered, test_data);
    }

    #[test]
    fn test_sensor_error_handling() -> Result<()> {
        // Test que los sensores manejan errores correctamente
        let mut lidar = Lidar::new("invalid_port".to_string(), 115200);

        // Inicialización debería fallar con puerto inválido
        assert!(lidar.initialize().is_err());

        Ok(())
    }
}

#[cfg(test)]
mod sensor_integration_tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_multiple_sensors_concurrent() -> Result<()> {
        // Test que múltiples sensores pueden operar concurrentemente
        let mut lidar = Lidar::new("/dev/ttyUSB0".to_string(), 115200);
        let mut camera = Camera::new(0, (640, 480), 30);
        let mut imu = IMU::new(0x68);

        tokio::try_join!(
            async { lidar.initialize() },
            async { camera.initialize() },
            async { imu.initialize() }
        )?;

        // Verificar que todos los sensores están conectados
        assert!(lidar.get_status().connected);
        assert!(camera.get_status().connected);
        assert!(imu.get_status().connected);

        Ok(())
    }
}
