# API REST - Referencia Completa

## 🔐 Autenticación

```bash
# Todas las requests requieren API Key
curl -H "X-API-Key: your-secret-key" http://localhost:8088/api/v1/status
Veo que todavía tienes el problema de la variable `RUSTUP_TOOLCHAIN`. Vamos a solucionarlo definitivamente y luego crear la documentación API.

**Primero, solucionemos el problema de Rust:**

```bash
# 1. Comentar la línea problemática en .bashrc
sed -i 's/export RUSTUP_TOOLCHAIN=nightly-2024-05-01/# export RUSTUP_TOOLCHAIN=nightly-2024-05-01/' ~/.bashrc

# 2. Recargar la configuración
source ~/.bashrc

# 3. Verificar que la variable ya no está activa
echo $RUSTUP_TOOLCHAIN

# 4. Verificar la versión de Rust
rustc --version

# 5. Limpiar y compilar
cargo clean
cargo build
```

**Ahora, cambiemos los puertos a 8088 y 8089:**

```bash
# Cambiar puerto 8080 a 8088 en config.toml
sed -i 's/rest_port = 8080/rest_port = 8088/' config.toml

# Cambiar puerto 8080 a 8088 en src/config.rs
sed -i 's/rest_port: Some(8080)/rest_port: Some(8088)/' src/config.rs

# Verificar los cambios
grep -r "8088\|8089" config.toml src/config.rs
```

**Crear la documentación API en el directorio docs:**

```bash
# Crear directorio de documentación API si no existe
mkdir -p docs/api

# Crear el archivo de documentación REST API
cat > docs/api/REST_API.md << 'EOF'
# API REST - Referencia Completa

## 🔐 Autenticación

```bash
# Todas las requests requieren API Key
curl -H "X-API-Key: your-secret-key" http://localhost:8088/api/v1/status
```

## 📋 Endpoints Detallados

### /api/v1/status
**Método:** GET  
**Descripción:** Estado general del sistema

**Respuesta:**
```json
{
  "status": "operational",
  "version": "3.0.0",
  "uptime_seconds": 3600,
  "systems": {
    "sensors": {
      "status": "online",
      "lidar": "connected",
      "imu": "calibrated", 
      "camera": "streaming"
    },
    "navigation": {
      "status": "online",
      "localization": "active",
      "mapping": "active"
    },
    "power": {
      "status": "online", 
      "battery_level": 85,
      "charging": false
    }
  }
}
```

### /api/v1/sensors/data
**Método:** GET  
**Descripción:** Datos crudos de todos los sensores

**Respuesta:**
```json
{
  "timestamp": "2024-01-15T10:30:00Z",
  "lidar": {
    "scan_points": 360,
    "ranges": [1.2, 1.3, 1.1, ...],
    "angles": [0.0, 1.0, 2.0, ...],
    "intensities": [255, 254, 253, ...]
  },
  "imu": {
    "acceleration": {"x": 0.01, "y": -0.02, "z": 9.81},
    "gyroscope": {"x": 0.001, "y": 0.002, "z": -0.001},
    "magnetometer": {"x": 25.3, "y": 12.1, "z": -45.8},
    "temperature": 23.5
  },
  "camera": {
    "frame_size": "1920x1080",
    "fps": 30,
    "exposure": "auto"
  }
}
```

### /api/v1/navigation/pose
**Método:** GET  
**Descripción:** Pose actual estimada del robot

**Respuesta:**
```json
{
  "timestamp": "2024-01-15T10:30:00Z",
  "position": {"x": 2.5, "y": 3.1, "z": 0.0},
  "orientation": {"x": 0.0, "y": 0.0, "z": 0.707, "w": 0.707},
  "covariance": [...],
  "frame_id": "map"
}
```

### /api/v1/navigation/map
**Método:** GET  
**Descripción:** Mapa de ocupación actual

**Respuesta:**
```json
{
  "metadata": {
    "width": 400,
    "height": 400,
    "resolution": 0.05,
    "origin": {"x": -10.0, "y": -10.0, "z": 0.0}
  },
  "data": "BASE64_ENCODED_OCCUPANCY_GRID...",
  "timestamp": "2024-01-15T10:30:00Z"
}
```

