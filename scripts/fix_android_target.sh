#!/bin/bash

echo "🎯 REPARANDO TARGET ANDROID NO DESEADO"
echo "======================================"

# 1. Eliminar configuraciones problemáticas
echo "🗑️ Eliminando configuraciones de Android..."
rm -rf .cargo/
rm -f rust-toolchain.toml rust-toolchain .config

# 2. Resetear Rust
echo "⚙️ Reseteando configuración Rust..."
rustup override unset
rustup default stable

# 3. Remover target Android
echo "🔧 Removiendo target Android..."
rustup target remove aarch64-linux-android 2>/dev/null || true

# 4. Agregar target correcto
echo "🎯 Configurando target Linux..."
rustup target add x86_64-unknown-linux-gnu

# 5. Verificar
echo "📋 Verificando configuración:"
echo "   Rust version: $(rustc --version)"
echo "   Default host: $(rustc -vV | grep host | cut -d' ' -f2)"
echo "   Active toolchain: $(rustup show active-toolchain)"

# 6. Limpiar y compilar
echo "🔄 Limpiando y compilando..."
cargo clean
cargo build --target x86_64-unknown-linux-gnu

if [ $? -eq 0 ]; then
    echo ""
    echo "🎉 ¡REPARACIÓN EXITOSA! 🎉"
    echo "🤖 Ejecuta: ./target/x86_64-unknown-linux-gnu/debug/mechbot-3x"
else
    echo "❌ Error en compilación"
    echo "📋 Intentando compilación normal..."
    cargo build
fi
