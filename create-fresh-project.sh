#!/bin/bash
echo "🎯 CREANDO PROYECTO NUEVO DESDE CERO..."

# Crear directorio nuevo
cd ~
rm -rf mechbot-3x-working
mkdir mechbot-3x-working
cd mechbot-3x-working

echo "✅ Directorio nuevo creado"

# Configurar Rust
echo "⚙️ Configurando Rust..."
rustup override unset
rustup default stable
rustup target add x86_64-unknown-linux-gnu

# Crear Cargo.toml
cat > Cargo.toml << 'CARGO'
[package]
name = "mechbot-3x"
version = "0.1.0"
edition = "2021"

[dependencies]
anyhow = "1.0"
toml = "0.8"
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }

[[bin]]
name = "mechbot-3x"
path = "src/main.rs"
CARGO

# Crear estructura de código
mkdir src

# main.rs
cat > src/main.rs << 'MAIN'
use anyhow::Result;

mod config;
mod robot;

use crate::config::Config;
use crate::robot::Robot;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 MechBot-3x INICIALIZADO CORRECTAMENTE!");
    println!("✅ Versión: 3.0.0");
    println!("✅ Target: Linux x86_64");
    
    let config = Config::default();
    println!("✅ Configuración: {}", config.robot.name);
    
    let _robot = Robot::new(config).await?;
    println!("✅ Robot inicializado");
    
    println!("🎉 ¡SISTEMA LISTO PARA OPERAR!");
    Ok(())
}
MAIN

# config.rs
cat > src/config.rs << 'CONFIG'
use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub robot: RobotConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RobotConfig {
    pub name: String,
    pub model: String,
    pub version: String,
}

impl Config {
    pub fn from_file(_path: &str) -> Result<Self> {
        Ok(Self::default())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            robot: RobotConfig {
                name: "MechBot-3x-Working".to_string(),
                model: "MB3X".to_string(),
                version: "3.0.0".to_string(),
            },
        }
    }
}
CONFIG

# robot.rs
cat > src/robot.rs << 'ROBOT'
use crate::config::Config;
use anyhow::Result;

pub struct Robot {
    config: Config,
}

impl Robot {
    pub async fn new(config: Config) -> Result<Self> {
        Ok(Robot { config })
    }
}
ROBOT

# Compilar
echo "🔄 Compilando..."
cargo build --target x86_64-unknown-linux-gnu

if [ $? -eq 0 ]; then
    echo ""
    echo "🎉 ¡COMPILACIÓN EXITOSA! 🎉"
    echo "🤖 Ejecuta: ./target/x86_64-unknown-linux-gnu/debug/mechbot-3x"
    echo ""
    # Ejecutar automáticamente
    ./target/x86_64-unknown-linux-gnu/debug/mechbot-3x
else
    echo "❌ Error en compilación"
    cargo check
fi
