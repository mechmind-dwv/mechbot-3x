#!/bin/bash
echo "🎯 SOLUCIÓN DEFINITIVA - Eliminando target Android forzado"

# 1. Eliminar configuraciones problemáticas
echo "🗑️ Eliminando configuraciones de toolchain..."
rm -f rust-toolchain.toml rust-toolchain 
rm -rf .cargo/

# 2. Resetear configuración de Rust
echo "⚙️ Reseteando Rust..."
rustup override unset
unset RUSTUP_TOOLCHAIN
unset CARGO_BUILD_TARGET

# 3. Configurar toolchain correcta
echo "🔧 Configurando toolchain stable..."
rustup default stable
rustup target add x86_64-unknown-linux-gnu

# 4. Verificar estado
echo "📋 Verificando configuración:"
echo "   Rust version: $(rustc --version)"
echo "   Default host: $(rustc -vV | grep host | cut -d' ' -f2)"
echo "   Active toolchain: $(rustup show active-toolchain)"

# 5. Compilar
echo "🔄 Compilando para x86_64 Linux..."
cargo clean
cargo build --target x86_64-unknown-linux-gnu

if [ $? -eq 0 ]; then
    echo ""
    echo "🎉 ¡COMPILACIÓN EXITOSA! 🎉"
    echo "🤖 Ejecuta: ./target/x86_64-unknown-linux-gnu/debug/mechbot-3x"
else
    echo "❌ Error en compilación"
    echo "📝 Mostrando errores detallados:"
    cargo check --target x86_64-unknown-linux-gnu
fi
