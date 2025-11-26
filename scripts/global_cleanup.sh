#!/bin/bash

echo "🌍 LIMPIEZA GLOBAL DE CONFIGURACIONES ANDROID"
echo "============================================="

# 1. Limpiar configuraciones globales problemáticas
echo "🗑️ Limpiando configuraciones globales..."
if [ -f ~/.cargo/config.toml ]; then
    echo "⚠️  Configuración global encontrada. Creando backup..."
    cp ~/.cargo/config.toml ~/.cargo/config.toml.backup
    # Eliminar solo líneas problemáticas, no el archivo completo
    grep -v "android\|aarch64" ~/.cargo/config.toml > ~/.cargo/config.toml.tmp
    mv ~/.cargo/config.toml.tmp ~/.cargo/config.toml
fi

# 2. Limpiar cache de Cargo
echo "🧹 Limpiando cache de Cargo..."
rm -rf ~/.cargo/registry/cache/*
rm -rf ~/.cargo/registry/src/*

# 3. Resetear configuración de proyecto
echo "⚙️ Reseteando proyecto..."
rm -rf .cargo/ target/ 
rustup override unset

# 4. Configurar proyecto específicamente para Linux
mkdir -p .cargo
cat > .cargo/config.toml << 'CONFIG'
[build]
target = "x86_64-unknown-linux-gnu"

[target.x86_64-unknown-linux-gnu]
linker = "cc"

# Forzar siempre Linux, ignorar cualquier otra configuración
[env]
CARGO_BUILD_TARGET = "x86_64-unknown-linux-gnu"
CONFIG

# 5. Compilar sin cache
echo "🔨 Compilando sin cache..."
CARGO_BUILD_TARGET="x86_64-unknown-linux-gnu" cargo build --target x86_64-unknown-linux-gnu

if [ $? -eq 0 ]; then
    echo ""
    echo "🎉 ¡LIMPIEZA GLOBAL EXITOSA! 🎉"
    echo "🤖 Binario: ./target/x86_64-unknown-linux-gnu/debug/mechbot-3x"
else
    echo "❌ Error persistente. Usando método alternativo..."
    # Método alternativo: compilar directamente sin target
    cargo clean
    cargo build
fi
