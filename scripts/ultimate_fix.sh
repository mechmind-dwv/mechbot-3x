#!/bin/bash

echo "🚀 SOLUCIÓN ULTIMATE MECHBOT-3X"

# 1. Backup de archivos críticos
cp src/lib.rs src/lib.rs.backup
cp src/main.rs src/main.rs.backup

# 2. Limpieza total
cargo clean
rm -f Cargo.lock

# 3. Versión mínima que compila
cat > src/lib_minimal.rs << 'LIB'
pub mod api;
pub mod config; 
pub mod control;
pub mod navigation;
pub mod sensors;
pub mod vision;

pub use config::Config;

pub async fn initialize_system(config: Config) -> anyhow::Result<()> {
    println!("🚀 Sistema inicializado con config: {}", config.robot.name);
    Ok(())
}
LIB

cat > src/main_minimal.rs << 'MAIN'
use anyhow::Result;
use mechbot_3x::initialize_system;

#[tokio::main] 
async fn main() -> Result<()> {
    println!("🎯 MechBot-3x Iniciando...");
    let config = mechbot_3x::Config::default();
    initialize_system(config).await?;
    println!("✅ Sistema ejecutándose correctamente!");
    Ok(())
}
MAIN

# 4. Reemplazar temporalmente
mv src/lib_minimal.rs src/lib.rs
mv src/main_minimal.rs src/main.rs

# 5. Compilar versión mínima
if cargo build; then
    echo "🎉 ¡VERSIÓN MÍNIMA COMPILA!"
    
    # 6. Restaurar archivos originales
    mv src/lib.rs.backup src/lib.rs
    mv src/main.rs.backup src/main.rs
    
    # 7. Ahora arreglar errores incrementalmente
    ./scripts/fix_compilation_errors.sh
else
    echo "❌ Error en versión mínima - problemas graves"
fi
