#!/bin/bash
# update_mechbot.sh

echo "🔄 Iniciando actualización de MechBot-3x..."

# Backup de configuración
cp config.toml config.toml.backup.$(date +%Y%m%d)

# Detener servicio
sudo systemctl stop mechbot

# Actualizar código
git pull origin main

# Recompilar
cargo build --release

# Verificar nueva versión
./target/release/mechbot-3x --version

# Restaurar configuración si es necesario
cp config.toml.backup.$(date +%Y%m%d) config.toml

# Iniciar servicio
sudo systemctl start mechbot

echo "✅ Actualización completada"
