# Integración de Sensores - Guía Práctica

## 🔌 Conexión Física

### Diagrama de Pines
```
SENSORES ──────────────────────────────────────
                    
LIDAR (USB)    IMU (I2C)      CÁMARA (USB)
   │              │                │
   ├─ /dev/ttyUSB0├─ SDA: GPIO2    ├─ /dev/video0  
   │              ├─ SCL: GPIO3    │
   │              ├─ 3.3V          │
   └─ GND         └─ GND           └─ GND
```

### Verificación de Hardware
```bash
#!/bin/bash
# script de verificación de sensores
echo "🔍 Verificando sensores..."

# Verificar LIDAR
if [ -c "/dev/ttyUSB0" ]; then
    echo "✅ LIDAR detectado: /dev/ttyUSB0"
else
    echo "❌ LIDAR no detectado"
fi

# Verificar IMU
if i2cdetect -y 1 | grep -q "68"; then
    echo "✅ IMU detectado en dirección 0x68"
else
    echo "❌ IMU no detectado"
fi

# Verificar cámara
if v4l2-ctl --list-devices | grep -q "video0"; then
    echo "✅ Cámara detectada: /dev/video0"
else
    echo "❌ Cámara no detectada"
fi
```

## 💻 Integración en Código

### Inicialización
```rust
use mechbot_3x::sensors::{Lidar, IMU, Camera, SensorManager};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Inicializar gestor de sensores
    let sensor_manager = SensorManager::new().await?;
    
    // Configurar LIDAR
    let lidar_config = LidarConfig {
        port: "/dev/ttyUSB0".to_string(),
        baudrate: 115200,
        ..Default::default()
    };
    
    // Configurar IMU
    let imu_config = IMUConfig {
        i2c_address: 0x68,
        sample_rate: 100,
        ..Default::default()
    };
    
    // Inicializar sensores
    sensor_manager.add_lidar(lidar_config).await?;
    sensor_manager.add_imu(imu_config).await?;
    sensor_manager.add_camera(CameraConfig::default()).await?;
    
    // Iniciar adquisición de datos
    sensor_manager.start().await?;
    
    Ok(())
}
```

### Lectura de Datos
```rust
// Lectura síncrona
let sensor_data = sensor_manager.read_all().await?;
println!("LIDAR points: {}", sensor_data.lidar_points.len());
println!("IMU acceleration: {:?}", sensor_data.imu.acceleration);

// Stream de datos en tiempo real
let mut sensor_stream = sensor_manager.stream();
while let Some(data) = sensor_stream.next().await {
    process_sensor_data(data).await?;
}
```

## 🧪 Ejemplos de Uso

### Navegación con LIDAR
```rust
async fn avoid_obstacles(sensors: &SensorManager) -> anyhow::Result<()> {
    loop {
        let data = sensors.read_all().await?;
        
        // Detectar obstáculos cercanos
        let close_obstacles: Vec<_> = data.lidar_points
            .iter()
            .filter(|p| p.distance < 1.0)
            .collect();
            
        if !close_obstacles.is_empty() {
            // Tomar acción evasiva
            avoid_collision(close_obstacles).await?;
        }
        
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}
```

### Fusión IMU + Odometría
```rust
async fn estimate_pose(sensors: &SensorManager) -> anyhow::Result<Pose> {
    let data = sensors.read_all().await?;
    
    // Fusión de datos IMU y odometría
    let fused_pose = pose_estimator.fuse(
        data.imu.acceleration,
        data.imu.gyroscope,
        data.odometry
    )?;
    
    Ok(fused_pose)
}
```

## 📊 Visualización de Datos

### Web Interface
```rust
// Servir datos de sensores via WebSocket
pub async fn serve_sensor_data(axum::extract::Ws(ws): axum::extract::Ws) {
    let mut sensor_stream = sensor_manager.stream();
    
    while let Some(data) = sensor_stream.next().await {
        let json_data = serde_json::to_string(&data).unwrap();
        ws.send(axum::extract::ws::Message::Text(json_data)).await?;
    }
}
```

### ROS Integration
```rust
// Publicar datos en topics ROS
pub async fn publish_to_ros(sensors: &SensorManager) -> anyhow::Result<()> {
    let ros_node = ros::Node::new("mechbot_sensors")?;
    let lidar_pub = ros_node.advertise::<LaserScan>("scan", 10)?;
    let imu_pub = ros_node.advertise::<Imu>("imu/data", 10)?;
    
    let mut sensor_stream = sensors.stream();
    while let Some(data) = sensor_stream.next().await {
        // Publicar datos LIDAR
        let laser_scan = convert_to_laserscan(&data.lidar_points);
        lidar_pub.publish(laser_scan)?;
        
        // Publicar datos IMU
        let imu_msg = convert_to_imu(&data.imu);
        imu_pub.publish(imu_msg)?;
    }
    
    Ok(())
}
```

## 🔧 Mantenimiento

### Calibración Periódica
```rust
// Script de calibración automática
pub async fn auto_calibrate(sensors: &mut SensorManager) -> anyhow::Result<()> {
    println!("🔧 Iniciando calibración automática...");
    
    // Calibrar IMU
    sensors.calibrate_imu().await?;
    println!("✅ IMU calibrado");
    
    // Calibrar LIDAR (offset angular)
    sensors.calibrate_lidar().await?;
    println!("✅ LIDAR calibrado");
    
    // Verificar calibración
    let calibration_report = sensors.verify_calibration().await?;
    println!("📊 Reporte de calibración: {:?}", calibration_report);
    
    Ok(())
}
```

### Health Monitoring
```rust
// Monitoreo de salud de sensores
pub async fn sensor_health_check(sensors: &SensorManager) -> anyhow::Result<SensorHealth> {
    let mut health = SensorHealth::new();
    
    // Verificar LIDAR
    match sensors.get_lidar_health().await {
        Ok(lidar_health) => health.lidar = lidar_health,
        Err(e) => health.lidar = SensorStatus::Error(e.to_string()),
    }
    
    // Verificar IMU
    match sensors.get_imu_health().await {
        Ok(imu_health) => health.imu = imu_health,
        Err(e) => health.imu = SensorStatus::Error(e.to_string()),
    }
    
    // Verificar cámara
    match sensors.get_camera_health().await {
        Ok(camera_health) => health.camera = camera_health,
        Err(e) => health.camera = SensorStatus::Error(e.to_string()),
    }
    
    Ok(health)
}
```
