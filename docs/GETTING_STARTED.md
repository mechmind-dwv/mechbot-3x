# 🚀 Guía de Inicio Rápido - MechBot-3x

Esta guía te ayudará a poner en marcha tu MechBot-3x en menos de 30 minutos.

## 📋 Prerrequisitos

### Hardware Requerido
- Computadora con Linux (Ubuntu 20.04+ recomendado)
- 8GB RAM mínimo, 16GB recomendado
- 50GB espacio en disco
- Puerto USB para sensores
- Conexión a internet

### Software Requerido
```bash
# Verificar instalaciones previas
rustc --version  # Rust 1.81+
cargo --version
python3 --version  # Python 3.8+
git --version
```

## ⚡ Instalación Express

### Paso 1: Clonar el Repositorio
```bash
git clone https://github.com/mechmind-dwv/mechbot-3x.git
cd mechbot-3x
```

### Paso 2: Instalar Dependencias del Sistema
```bash
# Ubuntu/Debian
sudo apt update
sudo apt install -y build-essential cmake pkg-config libopencv-dev \
    libusb-1.0-0-dev libssl-dev libasound2-dev

# Fedora
sudo dnf groupinstall "Development Tools"
sudo dnf install opencv-devel cmake pkgconfig
```

### Paso 3: Configurar Rust
```bash
# Si no tienes Rust instalado:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Configurar toolchain
rustup default stable
rustup component add rustfmt clippy
```

### Paso 4: Compilar el Proyecto
```bash
# Compilación inicial (puede tomar varios minutos)
cargo build --release

# Verificar que compiló correctamente
./target/release/mechbot-3x --version
```

## 🎯 Tu Primera Ejecución

### Configuración Básica
```bash
# Crear archivo de configuración mínimo
cat > config_minimal.toml << 'CONFIG'
[robot]
name = "Mi-MechBot"
model = "MB3X"
version = "3.0.0"

[sensors]
# Configuración básica sin hardware
camera_index = 0

[api]
rest_port = 8080
websocket_port = 8081

[logging]
level = "info"
output = "console"
CONFIG
```

### Ejecutar en Modo Simulación
```bash
# Ejecutar sin hardware conectado
./target/release/mechbot-3x --config config_minimal.toml --simulation
```

Deberías ver:
```
🚀 MechBot-3x inicializado!
✅ Configuración cargada: Mi-MechBot
🤖 Sistema operando en modo simulación
🌐 API REST disponible en: http://localhost:8080
🔌 WebSocket disponible en: ws://localhost:8081
```

## 🔌 Conexión de Hardware Básico

### Conectar Cámara USB
```bash
# Verificar que la cámara está detectada
ls -la /dev/video*
v4l2-ctl --list-devices

# Probar la cámara
ffmpeg -f v4l2 -i /dev/video0 -frames 1 test_camera.jpg
```

### Configuración con Cámara
```toml
[sensors]
camera_index = 0
camera_width = 1280
camera_height = 720
camera_fps = 30
```

## 🌐 Primera Interacción vía API

### Verificar Estado del Sistema
```bash
curl http://localhost:8080/api/v1/status
```

Respuesta esperada:
```json
{
  "status": "operational",
  "version": "3.0.0",
  "systems": {
    "sensors": "online",
    "navigation": "online",
    "vision": "online"
  }
}
```

### Interfaz Web Básica
```bash
# Si tienes el frontend, servir archivos estáticos
python3 -m http.server 3000
```

Luego visita: `http://localhost:3000`

## 🧪 Ejemplos Prácticos

### Ejemplo 1: Control Básico desde Python
```python
import requests
import json

class MechBotClient:
    def __init__(self, base_url="http://localhost:8080"):
        self.base_url = base_url
    
    def get_status(self):
        response = requests.get(f"{self.base_url}/api/v1/status")
        return response.json()
    
    def take_picture(self):
        response = requests.post(f"{self.base_url}/api/v1/vision/capture")
        return response.json()

# Uso
bot = MechBotClient()
status = bot.get_status()
print(f"Estado: {status['status']}")
```

### Ejemplo 2: Script de Monitoreo
```bash
#!/bin/bash
# monitoring_script.sh

echo "🔍 Monitoreando MechBot-3x..."

while true; do
    STATUS=$(curl -s http://localhost:8080/api/v1/status)
    BATTERY=$(echo $STATUS | grep -o '"battery_level":[0-9]*' | cut -d: -f2)
    
    echo "Batería: ${BATTERY}% - $(date)"
    
    if [ "$BATTERY" -lt 20 ]; then
        echo "⚠️  Batería baja! Conectar a carga."
    fi
    
    sleep 30
done
```

## 🎮 Comandos Útiles

### Comandos de Desarrollo
```bash
# Compilar en modo desarrollo (más rápido)
cargo build

# Ejecutar tests
cargo test

# Ver documentación
cargo doc --open

# Formatear código
cargo fmt

# Linter
cargo clippy
```

### Comandos de Sistema
```bash
# Ver logs en tiempo real
tail -f mechbot.log

# Ver uso de recursos
htop

# Monitorear puertos
netstat -tulpn | grep 8080
```

## 🔍 Verificación de Instalación

### Script de Verificación
```bash
cat > verify_installation.sh << 'VERIFY'
#!/bin/bash
echo "🔍 Verificando instalación de MechBot-3x..."

# Verificar Rust
if command -v rustc &> /dev/null; then
    echo "✅ Rust instalado: $(rustc --version)"
else
    echo "❌ Rust no instalado"
fi

# Verificar compilación
if [ -f "target/release/mechbot-3x" ]; then
    echo "✅ Binario compilado correctamente"
else
    echo "❌ Binario no encontrado, compilar con: cargo build --release"
fi

# Verificar configuración
if [ -f "config.toml" ]; then
    echo "✅ Archivo de configuración encontrado"
else
    echo "⚠️  Crear config.toml desde config_minimal.toml"
fi

# Verificar APIs
if curl -s http://localhost:8080/api/v1/status > /dev/null; then
    echo "✅ API REST funcionando"
else
    echo "❌ API REST no responde"
fi

echo "🎉 Verificación completada!"
VERIFY

chmod +x verify_installation.sh
./verify_installation.sh
```

## 📈 Próximos Pasos

1. **Explorar la API**: Prueba los diferentes endpoints
2. **Conectar más sensores**: LIDAR, IMU, etc.
3. **Personalizar configuración**: Modifica `config.toml` para tu hardware
4. **Desarrollar aplicaciones**: Usa el SDK para crear tus propias apps
5. **Unirse a la comunidad**: Discord y GitHub para soporte

## 🆘 ¿Problemas?

Si encuentras problemas:

1. Revisa `docs/TROUBLESHOOTING.md`
2. Verifica los logs: `tail -f mechbot.log`
3. Consulta issues en GitHub
4. Únete al Discord para ayuda en tiempo real

---

**¡Felicitaciones! 🎉 Ahora tienes MechBot-3x funcionando.**
El siguiente paso es explorar las capacidades avanzadas y conectar tu hardware específico.
