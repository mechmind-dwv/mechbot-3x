// Ejemplo de integración de sensores MechBot-3X
// Ejecutar con: cargo run --example sensor_integration

use futures::StreamExt;
use mechbot_3x::sensors::{CameraConfig, IMUConfig, LidarConfig, SensorManager};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔌 Sistema de Sensores MechBot-3X");
    println!("=================================");

    // Crear gestor de sensores
    let mut sensor_manager = SensorManager::new().await?;

    // Configurar y agregar sensores
    println!("📡 Inicializando sensores...");

    // LIDAR
    let lidar_config = LidarConfig {
        port: "/dev/ttyUSB0".to_string(),
        baudrate: 115200,
        ..Default::default()
    };
    sensor_manager.add_lidar(lidar_config).await?;

    // IMU
    let imu_config = IMUConfig {
        i2c_address: 0x68,
        sample_rate: 100,
        ..Default::default()
    };
    sensor_manager.add_imu(imu_config).await?;

    // Cámara
    let camera_config = CameraConfig::default();
    sensor_manager.add_camera(camera_config).await?;

    // Calibrar IMU
    println!("🔧 Calibrando IMU...");
    sensor_manager.calibrate_imu(100).await?;

    // Verificar estado de sensores
    let status = sensor_manager.get_sensor_status().await;
    println!("📊 Estado de sensores:");
    for (sensor_name, sensor_status) in status {
        println!("   - {}: {:?}", sensor_name, sensor_status.health);
    }

    // Iniciar adquisición de datos
    println!("🚀 Iniciando adquisición de datos...");
    sensor_manager.start().await?;

    // Leer datos por 5 segundos usando stream
    let mut sensor_stream = sensor_manager.stream();
    let timeout = tokio::time::sleep(tokio::time::Duration::from_secs(5));
    tokio::pin!(timeout);

    let mut sample_count = 0;

    loop {
        tokio::select! {
            Some(sensor_data) = sensor_stream.next() => {
                sample_count += 1;

                if sample_count % 10 == 0 { // Mostrar cada 10 muestras
                    println!("📊 Muestra {}:", sample_count);

                    if let Some(ref lidar) = sensor_data.lidar {
                        if let Some(closest) = lidar.find_closest_obstacle() {
                            println!("   - LIDAR: obstáculo más cercano: {:.2}m a {:.1}°",
                                    closest.distance, closest.angle.to_degrees());
                        }
                    }

                    if let Some(ref imu) = sensor_data.imu {
                        println!("   - IMU: acc={:.2}m/s², gyro={:.2}rad/s, temp={:.1}°C",
                                imu.get_acceleration_magnitude(),
                                imu.get_gyroscope_magnitude(),
                                imu.temperature);
                    }

                    if let Some(ref camera) = sensor_data.camera {
                        println!("   - Cámara: {}x{}, {} bytes",
                                camera.width, camera.height, camera.data.len());
                    }
                }
            }
            _ = &mut timeout => {
                println!("⏰ Timeout alcanzado");
                break;
            }
        }
    }

    // Detener sensores
    sensor_manager.stop().await?;

    println!("✅ Recibidas {} muestras de sensores", sample_count);
    println!("🎉 Integración de sensores completada exitosamente");

    Ok(())
}
