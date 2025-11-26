#!/bin/bash
echo "🔧 Reparando imports de main.rs..."

# Backup de archivos originales
cp src/main.rs src/main.rs.backup 2>/dev/null || true
cp src/config.rs src/config.rs.backup 2>/dev/null || true

# Crear main.rs corregido
cat > src/main.rs << 'MAIN_EOF'
use anyhow::Result;

mod config;
mod robot;

use crate::config::Config;
use crate::robot::Robot;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 MechBot-3x inicializado!");
    
    let config = match Config::from_file("config.toml") {
        Ok(cfg) => {
            println!("✅ Configuración cargada correctamente");
            cfg
        }
        Err(e) => {
            eprintln!("❌ Error cargando configuración: {}", e);
            Config::default()
        }
    };
    
    let robot = Robot::new(config).await?;
    println!("✅ Robot inicializado correctamente");
    println!("🤖 MechBot-3x listo para operar!");
    
    Ok(())
}
MAIN_EOF

# Crear robot.rs básico
cat > src/robot.rs << 'ROBOT_EOF'
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
ROBOT_EOF

echo "✅ Archivos corregidos"
echo "🔄 Compilando..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "🎉 ¡Compilación exitosa!"
    echo "🤖 Ejecuta: ./target/release/mechbot-3x"
else
    echo "❌ Error en compilación, revisa los mensajes"
fi
