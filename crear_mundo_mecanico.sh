#!/bin/bash
echo "🌌 CREANDO MUNDO MECBOT-3X COMPLETO..."
echo "====================================="

# 1. Crear estructura de directorios
echo "📁 Creando estructura de directorios..."
mkdir -p logs data/models data/maps target/debug target/release

# 2. Crear archivos de configuración
echo "⚙️ Creando archivos de configuración..."

# Cargo.toml
cat > Cargo.toml << 'CARGO'
[package]
name = "mechbot-3x"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "mechbot-3x"
path = "src/main.rs"

[dependencies]
anyhow = "1.0"
toml = "0.8"
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }
log = "0.4"
env_logger = "0.10"

[dev-dependencies]
tokio = { version = "1.0", features = ["full"] }
CARGO

# config.toml
cat > config.toml << 'CONFIG'
[robot]
name = "MechBot-3x-Cosmic"
model = "MB3X-COSMIC"
version = "3.0.0"

[api]
rest_port = 8080
websocket_port = 8081

[logging]
level = "info"
CONFIG

echo "✅ Archivos de configuración creados"

# 3. Compilar proyecto
echo "🔧 Compilando MechBot-3x..."
cargo build --release

# 4. Verificar y ejecutar
if [ -f "./target/release/mechbot-3x" ]; then
    echo "🎉 ¡MECHBOT-3X CREADO EXITOSAMENTE!"
    echo "🚀 EJECUTANDO SISTEMA..."
    echo "========================"
    ./target/release/mechbot-3x
else
    echo "❌ No se pudo crear el binario"
    echo "💡 Verificando estructura..."
    find . -name "*.rs" | head -10
fi
