#!/bin/bash

echo "🎯 CONFIGURACIÓN INTERACTIVA MECHBOT-3X"
echo "========================================"

# Verificar Rust
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust no encontrado. Instalando..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source ~/.cargo/env
else
    echo "✅ Rust encontrado: $(rustc --version)"
fi

# Configuración del robot
read -p "🤖 Nombre del robot: " robot_name
robot_name=${robot_name:-"MechBot-Personalizado"}

read -p "🔧 Modelo: " robot_model
robot_model=${robot_model:-"MB3X"}

read -p "📊 Puerto API REST (8080): " api_port
api_port=${api_port:-8080}

read -p "🔌 Puerto WebSocket (8081): " ws_port
ws_port=${ws_port:-8081}

read -p "📈 Umbral de sensor (100.0): " sensor_threshold
sensor_threshold=${sensor_threshold:-100.0}

# Crear configuración
cat > config.toml << CONFIG
[robot]
name = "$robot_name"
model = "$robot_model"
version = "3.0.0"

[sensors]
lidar_port = "/dev/ttyUSB0"
camera_index = 0
sensor_threshold = $sensor_threshold

[api]
rest_port = $api_port
websocket_port = $ws_port
api_key = ""

[logging]
level = "info"
output = "console"
CONFIG

echo ""
echo "✅ Configuración guardada en config.toml"
echo "🤖 Robot: $robot_name"
echo "🌐 API: http://localhost:$api_port"
echo "🔌 WebSocket: ws://localhost:$ws_port"

# Preguntar si compilar
read -p "¿Compilar el proyecto ahora? (s/n): " -n 1 -r
echo
if [[ $REPLY =~ ^[Ss]$ ]]; then
    echo "🔄 Compilando..."
    cargo build --release
    
    if [ $? -eq 0 ]; then
        echo "🎉 ¡Compilación exitosa!"
        echo "🚀 Ejecuta: ./target/release/mechbot-3x"
    else
        echo "❌ Error en compilación"
    fi
fi
