# Especificaciones de Hardware - MechBot-3x

## 🖥️ Computadora Principal

### Requisitos Mínimos
- **CPU**: x86_64, 4+ cores, 2.0+ GHz
- **RAM**: 8GB DDR4
- **Almacenamiento**: 32GB SSD
- **USB**: 4+ puertos USB 3.0

### Recomendado
- **CPU**: Intel i7 / AMD Ryzen 7, 8+ cores
- **RAM**: 16GB DDR4
- **Almacenamiento**: 512GB NVMe SSD
- **GPU**: NVIDIA Jetson Orin (opcional para AI)

## 📡 Sensores

### LIDAR
- **Modelo**: RPLIDAR A1 / SLAMTEC RPLIDAR
- **Alcance**: 12 metros
- **Resolución angular**: 1°
- **Frecuencia**: 5.5 Hz
- **Interfaz**: USB 2.0

### Cámara
- **Resolución**: 1080p (1920x1080)
- **FPS**: 30-60
- **Interfaz**: USB 3.0
- **Campo de visión**: 120° diagonal

### IMU (Inertial Measurement Unit)
- **Acelerómetro**: ±8g
- **Giroscopio**: ±2000°/s  
- **Magnetómetro**: ±8 Gauss
- **Interfaz**: I2C

### Sensores Adicionales
- **Encoder motores**: 12 CPR
- **Sensor ultrasonido**: HC-SR04
- **Sensor infrarrojo**: GP2Y0A21YK
- **GPS**: U-blox NEO-6M (opcional)

## 🔌 Actuadores

### Motores DC
- **Voltaje**: 12V DC
- **Corriente**: 3A max por motor
- **Reducción**: Relación 30:1
- **Encoder**: 12 pulsos por revolución

### Controlador de Motores
- **Modelo**: L298N / TB6612FNG
- **Voltaje**: 5-46V DC
- **Corriente**: 3A por canal
- **Interfaz**: PWM + Digital

### Servomotores
- **Torque**: 20 kg/cm
- **Velocidad**: 0.15 seg/60°
- **Voltaje**: 6-7.4V
- **Control**: PWM 50Hz

## 🔋 Sistema de Potencia

### Batería
- **Química**: LiPo / Li-ion
- **Voltaje**: 12V nominal
- **Capacidad**: 5000-10000 mAh
- **Descarga**: 20C constante

### Regulación de Potencia
- **Step-down**: 12V to 5V (3A)
- **Step-down**: 5V to 3.3V (1A)
- **Protections**: Over-current, over-voltage, reverse polarity

## 🌐 Conectividad

### Red
- **Ethernet**: Gigabit (opcional)
- **WiFi**: 802.11ac 5GHz
- **Bluetooth**: 4.2+ (para control remoto)

### Comunicación
- **UART**: 2x puertos (GPS, debug)
- **I2C**: 1x bus (sensores)
- **SPI**: 1x bus (display, SD card)
- **GPIO**: 16+ pines

## 🔧 Esquemáticos y PCB

### Diagrama de Bloques
```
┌─────────────────┐    ┌─────────────────┐
│   Computadora   │◄──►│  Control Motor  │──► Motores
│    Principal    │    │      L298N      │
└─────────────────┘    └─────────────────┘
         │                       │
         │ I2C                   │ PWM
         ▼                       ▼
┌─────────────────┐    ┌─────────────────┐
│      IMU        │    │    Servomotores │
│   MPU-6050      │    │      SG90       │
└─────────────────┘    └─────────────────┘
         │
         │ USB
         ▼
┌─────────────────┐
│     LIDAR       │
│   RPLIDAR A1    │
└─────────────────┘
```

### Pinout Principal

| Pin | Función | Descripción |
|-----|---------|-------------|
| 3.3V | Power | 3.3V regulado |
| 5V | Power | 5V regulado |
| 12V | Power | Batería directo |
| GND | Ground | Tierra común |
| GPIO2 | I2C SDA | Datos I2C |
| GPIO3 | I2C SCL | Clock I2C |
| GPIO18 | PWM Motor A | Control velocidad motor A |
| GPIO19 | PWM Motor B | Control velocidad motor B |
| GPIO23 | DIR Motor A | Dirección motor A |
| GPIO24 | DIR Motor B | Dirección motor B |

## 🛠️ Montaje Mecánico

### Chasis
- **Material**: Aluminio 3mm / PLA+ impreso 3D
- **Dimensiones**: 300x250x150mm
- **Peso**: 2.5-3.5 kg

### Ruedas
- **Diámetro**: 100mm
- **Material**: Goma silicona
- **Tracción**: Buena en interiores

### Soporte Sensores
- **LIDAR**: Montaje superior, altura 200mm
- **Cámara**: Montaje frontal, ajustable
- **IMU**: Centro de masa del robot

## 🔌 Diagramas de Cableado

### Conexión LIDAR
```
RPLIDAR A1 ──── USB ──── Computadora
```

### Conexión IMU
```
MPU-6050 ──── I2C ──── GPIO (SDA:2, SCL:3)
          3.3V ──── 3.3V
          GND ──── GND
```

### Conexión Controlador Motor
```
L298N ──── Motor A+ ──── Motor Izquierdo
      ├─── Motor A- ──── Motor Izquierdo
      ├─── Motor B+ ──── Motor Derecho  
      ├─── Motor B- ──── Motor Derecho
      ├─── ENA ──── GPIO18 (PWM)
      ├─── IN1 ──── GPIO23 (DIR A)
      ├─── IN2 ──── GPIO23 (DIR A inversa)
      ├─── IN3 ──── GPIO24 (DIR B)
      ├─── IN4 ──── GPIO24 (DIR B inversa)
      ├─── 12V ──── Batería
      └─── GND ──── GND común
```

## ⚡ Consideraciones de Potencia

### Consumo Estimado
| Componente | Voltaje | Corriente | Potencia |
|------------|---------|-----------|----------|
| Computadora | 5V | 2A | 10W |
| LIDAR | 5V | 0.5A | 2.5W |
| Motores (x2) | 12V | 3A c/u | 72W max |
| IMU + Sensores | 3.3V | 0.1A | 0.33W |
| **Total** | | | **~85W max** |

### Autonomía
- **Batería 5000mAh**: ~45 minutos
- **Batería 10000mAh**: ~1.5 horas

## 🛡️ Seguridad y Protecciones

### Eléctricas
- Fusibles en líneas de potencia
- Diodos de protección contra retroalimentación
- Reguladores con limitación de corriente

### Mecánicas
- Parachoques con sensores de contacto
- Topes físicos para límites de movimiento
- Estructura resistente a impactos menores

## 🔍 Troubleshooting

### Problemas Comunes

#### LIDAR no detecta
- Verificar conexión USB
- Comprobar alimentación 5V
- Revisar permisos /dev/ttyUSB*

#### Motores no responden
- Verificar voltaje batería > 11V
- Comprobar conexiones GPIO
- Revisar configuración PWM

#### IMU da lecturas erróneas
- Verificar calibración
- Comprobar fuente de interferencias
- Revisar conexión I2C

## 📦 Part List Completa

| Componente | Modelo | Cantidad | Notas |
|------------|--------|----------|-------|
| Computadora | Raspberry Pi 4 | 1 | 4GB+ RAM |
| LIDAR | RPLIDAR A1 | 1 | Incluye USB |
| IMU | MPU-6050 | 1 | Incluye breakout |
| Controlador Motor | L298N | 1 | Doble puente H |
| Motores DC | JG-37GM | 2 | Con encoder |
| Batería | LiPo 3S | 1 | 5000mAh+ |
| Regulador | LM2596 | 2 | Step-down |
| Cámara | Logitech C920 | 1 | 1080p |

---

**¿Preguntas sobre hardware?** Contacta al equipo en [ia.mechmind@gmail.com](mailto:ia.mechmind@gmail.com)
