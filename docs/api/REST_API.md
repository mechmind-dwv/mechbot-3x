# API REST - Referencia Completa

## 🔐 Autenticación

```bash
# Todas las requests requieren API Key
curl -H "X-API-Key: your-secret-key" http://localhost:8080/api/v1/status
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

### /api/v1/control/move
**Método:** POST  
**Descripción:** Comando de movimiento a coordenadas específicas

**Body:**
```json
{
  "target": {"x": 5.0, "y": 3.0, "theta": 1.57},
  "speed": {"linear": 0.5, "angular": 0.3},
  "timeout_seconds": 30,
  "obstacle_avoidance": true,
  "task_id": "move_12345"
}
```

**Respuesta:**
```json
{
  "success": true,
  "task_id": "move_12345",
  "estimated_duration": 12.5,
  "path_length": 6.2
}
```

### /api/v1/control/velocity
**Método:** POST  
**Descripción:** Control directo de velocidad

**Body:**
```json
{
  "linear": {"x": 0.3, "y": 0.0, "z": 0.0},
  "angular": {"x": 0.0, "y": 0.0, "z": 0.1}
}
```

### /api/v1/control/stop
**Método:** POST  
**Descripción:** Parada de emergencia o normal

**Body:**
```json
{
  "emergency": false,
  "reason": "user_request"
}
```

### /api/v1/vision/detect
**Método:** POST  
**Descripción:** Detección de objetos en frame actual

**Respuesta:**
```json
{
  "detections": [
    {
      "class_id": 0,
      "class_name": "person",
      "confidence": 0.87,
      "bbox": {"x": 100, "y": 150, "width": 50, "height": 180},
      "position": {"x": 2.1, "y": 1.5, "z": 0.0}
    }
  ],
  "processing_time_ms": 45.2
}
```

### /api/v1/system/reboot
**Método:** POST  
**Descripción:** Reinicio controlado del sistema

**Body:**
```json
{
  "reason": "software_update",
  "delay_seconds": 10
}
```

### /api/v1/logs
**Método:** GET  
**Descripción:** Obtención de logs del sistema

**Parámetros Query:**
- `level` (opcional): error, warn, info, debug
- `since` (opcional): timestamp ISO
- `limit` (opcional): número de entradas

**Respuesta:**
```json
{
  "logs": [
    {
      "timestamp": "2024-01-15T10:29:55Z",
      "level": "info",
      "module": "navigation",
      "message": "Path planning completed",
      "data": {"path_length": 5.3, "planning_time": 0.12}
    }
  ]
}
```

## 🚨 Códigos de Error HTTP

| Código | Error | Descripción |
|--------|-------|-------------|
| 200 | OK | Request exitosa |
| 400 | Bad Request | Parámetros inválidos |
| 401 | Unauthorized | API Key missing o inválida |
| 403 | Forbidden | Permisos insuficientes |
| 404 | Not Found | Recurso no existe |
| 409 | Conflict | Estado incompatible |
| 422 | Unprocessable Entity | Datos inválidos |
| 429 | Too Many Requests | Rate limiting |
| 500 | Internal Server Error | Error del servidor |
| 503 | Service Unavailable | Sistema no listo |

## 📊 Ejemplos de Uso

### Python Client
```python
import requests
import json

class MechBotClient:
    def __init__(self, base_url="http://localhost:8080", api_key="your-key"):
        self.base_url = base_url
        self.headers = {
            "X-API-Key": api_key,
            "Content-Type": "application/json"
        }
    
    def get_status(self):
        response = requests.get(
            f"{self.base_url}/api/v1/status",
            headers=self.headers
        )
        return response.json()
    
    def move_to(self, x, y, theta=0.0):
        data = {
            "target": {"x": x, "y": y, "theta": theta},
            "speed": {"linear": 0.5, "angular": 0.3},
            "obstacle_avoidance": True
        }
        response = requests.post(
            f"{self.base_url}/api/v1/control/move",
            headers=self.headers,
            data=json.dumps(data)
        )
        return response.json()
    
    def get_sensor_data(self):
        response = requests.get(
            f"{self.base_url}/api/v1/sensors/data", 
            headers=self.headers
        )
        return response.json()

# Uso
bot = MechBotClient()
status = bot.get_status()
print(f"Battery: {status['systems']['power']['battery_level']}%")
```

### cURL Examples
```bash
# Get status
curl -H "X-API-Key: secret" http://localhost:8080/api/v1/status

# Move to position
curl -X POST -H "X-API-Key: secret" -H "Content-Type: application/json" \
  -d '{"target":{"x":5,"y":3}, "speed":{"linear":0.5}}' \
  http://localhost:8080/api/v1/control/move

# Get logs
curl -H "X-API-Key: secret" \
  "http://localhost:8080/api/v1/logs?level=error&limit=10"
```

## 🔐 Seguridad

### API Keys
- Generar keys únicas por cliente
- Rotar keys periódicamente
- Registrar uso para auditoría

### Rate Limiting
- 1000 requests por hora por API key
- Límites separados por endpoint
- Headers de rate limiting en respuestas

### HTTPS
- Siempre usar HTTPS en producción
- Certificados TLS válidos
- HSTS habilitado
