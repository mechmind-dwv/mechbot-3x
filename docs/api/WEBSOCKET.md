# WebSocket API - Comunicación en Tiempo Real

## 🔌 Conexión WebSocket

```javascript
// Conectar al WebSocket
const ws = new WebSocket('ws://localhost:8089/ws');

ws.onopen = () => {
    console.log('Conectado al MechBot-3X');
};

ws.onmessage = (event) => {
    const data = JSON.parse(event.data);
    console.log('Mensaje recibido:', data);
};
```

## 📡 Canales de Datos

### Sensor Data Stream
```json
{
  "type": "sensor_data",
  "timestamp": "2024-01-15T10:30:00Z",
  "data": {
    "lidar": {
      "scan_points": 360,
      "ranges": [1.2, 1.3, 1.1, ...]
    },
    "imu": {
      "acceleration": {"x": 0.01, "y": -0.02, "z": 9.81}
    }
  }
}
```

### Navigation Updates
```json
{
  "type": "navigation_update", 
  "timestamp": "2024-01-15T10:30:00Z",
  "data": {
    "position": {"x": 2.5, "y": 3.1, "z": 0.0},
    "velocity": {"linear": 0.3, "angular": 0.1},
    "goal": {"x": 5.0, "y": 3.0},
    "path": [[2.5, 3.1], [3.0, 3.1], [4.0, 3.0], [5.0, 3.0]]
  }
}
```

### System Alerts
```json
{
  "type": "alert",
  "level": "warning", // info, warning, error, critical
  "message": "Battery level below 20%",
  "timestamp": "2024-01-15T10:30:00Z",
  "data": {
    "battery_level": 18,
    "estimated_remaining_minutes": 15
  }
}
```

## 📤 Comandos por WebSocket

### Envío de Comandos
```json
{
  "command": "move_to",
  "id": "cmd_12345",
  "params": {
    "target": {"x": 5.0, "y": 3.0, "theta": 1.57},
    "speed": {"linear": 0.5, "angular": 0.3}
  }
}
```

### Respuesta a Comandos
```json
{
  "response_to": "cmd_12345",
  "status": "accepted", // accepted, rejected, completed, error
  "message": "Command accepted",
  "data": {
    "estimated_duration": 12.5
  }
}
```

## 🎯 Ejemplos de Uso

### JavaScript Client
```javascript
class MechBotWebSocket {
    constructor(url = 'ws://localhost:8089/ws') {
        this.url = url;
        this.ws = null;
        this.callbacks = new Map();
    }
    
    connect() {
        this.ws = new WebSocket(this.url);
        
        this.ws.onopen = () => {
            console.log('Connected to MechBot-3X');
        };
        
        this.ws.onmessage = (event) => {
            const data = JSON.parse(event.data);
            this.handleMessage(data);
        };
        
        this.ws.onclose = () => {
            console.log('Disconnected from MechBot-3X');
        };
    }
    
    sendCommand(command, params) {
        const commandId = 'cmd_' + Date.now();
        const message = {
            command: command,
            id: commandId,
            params: params
        };
        
        this.ws.send(JSON.stringify(message));
        return commandId;
    }
    
    moveTo(x, y, theta = 0) {
        return this.sendCommand('move_to', {
            target: {x, y, theta},
            speed: {linear: 0.5, angular: 0.3}
        });
    }
    
    setVelocity(linear, angular) {
        return this.sendCommand('set_velocity', {
            linear: linear,
            angular: angular
        });
    }
    
    handleMessage(data) {
        // Procesar diferentes tipos de mensajes
        switch(data.type) {
            case 'sensor_data':
                this.onSensorData(data.data);
                break;
            case 'navigation_update':
                this.onNavigationUpdate(data.data);
                break;
            case 'alert':
                this.onAlert(data);
                break;
        }
    }
    
    onSensorData(data) {
        // Actualizar UI con datos de sensores
        console.log('Sensor data:', data);
    }
    
    onNavigationUpdate(data) {
        // Actualizar posición en mapa
        console.log('Navigation update:', data);
    }
    
    onAlert(alert) {
        // Mostrar alerta al usuario
        console.log(`ALERT [${alert.level}]: ${alert.message}`);
    }
}

// Uso
const bot = new MechBotWebSocket();
bot.connect();

// Enviar comando después de conectar
setTimeout(() => {
    bot.moveTo(5, 3);
}, 1000);
```

### Python WebSocket Client
```python
import asyncio
import websockets
import json

class MechBotWebSocketClient:
    def __init__(self, uri="ws://localhost:8089/ws"):
        self.uri = uri
        self.websocket = None
    
    async def connect(self):
        self.websocket = await websockets.connect(self.uri)
        print("Connected to MechBot-3X")
        
        # Start listening for messages
        asyncio.create_task(self.listen())
    
    async def listen(self):
        async for message in self.websocket:
            data = json.loads(message)
            await self.handle_message(data)
    
    async def handle_message(self, data):
        message_type = data.get('type')
        if message_type == 'sensor_data':
            await self.on_sensor_data(data['data'])
        elif message_type == 'navigation_update':
            await self.on_navigation_update(data['data'])
        elif message_type == 'alert':
            await self.on_alert(data)
    
    async def send_command(self, command, params):
        command_msg = {
            'command': command,
            'id': f'cmd_{asyncio.get_event_loop().time()}',
            'params': params
        }
        await self.websocket.send(json.dumps(command_msg))
        return command_msg['id']
    
    async def move_to(self, x, y, theta=0.0):
        params = {
            'target': {'x': x, 'y': y, 'theta': theta},
            'speed': {'linear': 0.5, 'angular': 0.3}
        }
        return await self.send_command('move_to', params)
    
    async def on_sensor_data(self, data):
        print(f"Sensor data: {data}")
    
    async def on_navigation_update(self, data):
        print(f"Navigation update: {data}")
    
    async def on_alert(self, alert):
        print(f"ALERT [{alert['level']}]: {alert['message']}")

# Uso
async def main():
    client = MechBotWebSocketClient()
    await client.connect()
    await client.move_to(5.0, 3.0)
    
    # Mantener conexión
    await asyncio.Future()

# asyncio.run(main())
