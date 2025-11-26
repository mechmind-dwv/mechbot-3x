#!/bin/bash
# rollback.sh

echo "🔄 Ejecutando rollback..."

sudo systemctl stop mechbot

# Restaurar backup anterior
cp config.toml.backup.$(date -d "yesterday" +%Y%m%d) config.toml
git checkout HEAD~1

cargo build --release
sudo systemctl start mechbot

echo "✅ Rollback completado"
