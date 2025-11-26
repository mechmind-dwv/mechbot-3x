#!/bin/bash

echo "💥 REPARACIÓN NUCLEAR - ELIMINANDO CONFIGURACIONES ANDROID"
echo "=========================================================="

# 1. Eliminar TODAS las configuraciones
echo "🗑️ Eliminando configuraciones..."
rm -rf .cargo/ .rustup/ .config/ target/ 2>/dev/null || true

# 2. Resetear Rust completamente
echo "⚙️ Reseteando Rust..."
rustup override unset
rustup default stable
rustup target remove aarch64-linux-android 2>/dev/null || true

# 3. Forzar variables de entorno
echo "🎯 Forzando target Linux..."
export RUSTFLAGS="--target x86_64-unknown-linux-gnu"
export CARGO_BUILD_TARGET="x86_64-unknown-linux-gnu"
export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER="cc"

# 4. Crear .cargo/config.toml correcto
mkdir -p .cargo
cat > .cargo/config.toml << 'CONFIG'
[build]
target = "x86_64-unknown-linux-gnu"

[target.x86_64-unknown-linux-gnu]
linker = "cc"
CONFIG

# 5. Compilar
echo "🔨 Compilando..."
cargo clean
cargo build

if [ $? -eq 0 ]; then
    echo ""
    echo "🎉 ¡REPARACIÓN NUCLEAR EXITOSA! 🎉"
    echo "🤖 Binario: ./target/debug/mechbot-3x"
    
    # Verificar que el binario es ejecutable
    if [ -f "target/debug/mechbot-3x" ]; then
        echo "✅ Binario creado correctamente"
        file target/debug/mechbot-3x
    fi
else
    echo "❌ Error en compilación"
    echo "📋 Último intento con flags explícitos..."
    cargo build --target x86_64-unknown-linux-gnu
fi
