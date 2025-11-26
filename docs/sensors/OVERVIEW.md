# Sistema de Sensores - MechBot-3x

## 🎯 Visión General

El sistema de sensores proporciona percepción del entorno en tiempo real usando múltiples modalidades de sensores fusionados.

## 📡 Sensores Implementados

### LIDAR
```rust
pub struct LidarSensor {
    pub port: String,
    pub baudrate: u32,
    pub max_range: f64,
    pub min_range: f64,
}

impl LidarSensor {
    pub async fn new(config: &LidarConfig) -> Result<Self> {
        // Inicialización del LIDAR
    }
    
    pub async fn scan(&mut self) -> Result<Vec<Point>> {
        // Realizar escaneo
    }
}
```

### IMU (Unidad de Medición Inercial)
```rust
pub struct IMUSensor {
    pub i2c_address: u8,
    pub sample_rate: u32,
}

impl IMUSensor {
    pub async fn read_accelerometer(&self) -> Result<Vector3<f64>>;
    pub async fn read_gyroscope(&self) -> Result<Vector3<f64>>;
    pub async fn read_magnetometer(&self) -> Result<Vector3<f64>>;
}
```

### Cámara
```rust
pub struct Camera {
    pub index: u32,
    pub resolution: (u32, u32),
    pub fps: u32,
}

impl Camera {
    pub async fn capture_frame(&mut self) -> Result<Frame>;
    pub async fn start_stream(&mut self) -> Result<()>;
}
```

## 🔄 Fusión de Sensores

### Filtro de Kalman
```rust
pub struct SensorFusion {
    kalman_filter: KalmanFilter,
    last_update: Instant,
}

impl SensorFusion {
    pub fn update_imu(&mut self, accel: Vector3<f64>, gyro: Vector3<f64>);
    pub fn update_lidar(&mut self, position: Point);
    pub fn get_pose(&self) -> Pose;
}
```

### Calibración
```rust
pub trait Calibratable {
    fn calibrate(&mut self) -> Result<()>;
    fn is_calibrated(&self) -> bool;
}

impl Calibratable for IMUSensor {
    fn calibrate(&mut self) -> Result<()> {
        // Calibración de offsets del IMU
    }
}
```

## 🎚️ Configuración

### config.toml
```toml
[sensors.lidar]
port = "/dev/ttyUSB0"
baudrate = 115200
max_range = 12.0
min_range = 0.15

[sensors.imu]
i2c_address = 0x68
sample_rate = 100
accel_range = 8
gyro_range = 2000

[sensors.camera]
index = 0
width = 1920
height = 1080
fps = 30
auto_exposure = true
```

## 🚨 Troubleshooting

### LIDAR
```bash
# Verificar dispositivo
ls -la /dev/ttyUSB*

# Probar comunicación
rosrun rplidar_ros rplidarNodeClient

# Ver logs
journalctl -u mechbot-sensors
```

### IMU
```bash
# Detectar dispositivos I2C
i2cdetect -y 1

# Probar comunicación
i2cget -y 1 0x68 0x75
```

### Cámara
```bash
# Listar cámaras
v4l2-ctl --list-devices

# Probar cámara
ffmpeg -f v4l2 -i /dev/video0 -frames 1 test.jpg
```
