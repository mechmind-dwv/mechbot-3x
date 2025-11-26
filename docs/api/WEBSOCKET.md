# WebSocket API - Comunicación en Tiempo Real

## 🔌 Conexión

```javascript
const ws = new WebSocket('ws://localhost:8081/ws');
```

## 📨 Mensajes de Entrada (Client → Server)

### Comando de Movimiento
```json
{
  "type": "move_command",
  "id": "cmd_123",
  "linear": {"x": 0.5, "y": 0.0, "z": 0.0},
  "angular": {"x": 0.0, "y": 0.0, "z": 0.2},
  "timestamp": 1673789400
}
```

### Configuración de Telemetría
```json
{
  "type": "telemetry_config", 
  "update_rate_hz": 10,
  "sensors": ["lidar", "imu", "pose"],
  "compression": true
}
```

## 📤 Mensajes de Salida (Server → Client)

### Telemetría
```json
{
  "type": "telemetry",
  "timestamp": 1673789400,
  "pose": {
    "position": {"x": 2.1, "y": 3.4, "z": 0.0},
    "orientation": {"x": 0.0, "y": 0.0, "z": 0.7, "w": 0.7},
    "covariance": [...]
  },
  "sensors": {
    "lidar": {"points": 360, "ranges": [...]},
    "imu": {"acceleration": {...}, "gyroscope": {...}}
  },
  "battery": {"level": 85, "voltage": 12.3}
}
```

### Estado del Sistema
```json
{
  "type": "system_status",
  "status": "operational",
  "components": {
    "sensors": "online",
    "navigation": "online",
    "vision": "online"
  },
  "alerts": []
}
```

### Logs en Tiempo Real
```json
{
  "type": "log_message",
  "level": "info",
  "module": "navigation",
  "message": "Path planning completed",
  "timestamp": 1673789400
}
```
