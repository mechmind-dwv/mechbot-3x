## **📖 Documentación**

### **API Endpoints**

#### **REST API**

```bash
# Estado del robot
GET /api/v1/status

# Mover a coordenadas
POST /api/v1/move
{
  "x": 100.0,
  "y": 200.0,
  "speed": 1.5
}

# Obtener mapa actual
GET /api/v1/map

# Datos de sensores
GET /api/v1/sensors
```

#### **WebSocket**

```javascript
// Conectar a telemetría en tiempo real
const ws = new WebSocket('ws://localhost:8081/telemetry');

ws.onmessage = (event) => {
  const data = JSON.parse(event.data);
  console.log('Position:', data.position);
  console.log('Sensors:', data.sensors);
};
```

