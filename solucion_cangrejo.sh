#!/bin/bash
echo "🦀 SOLUCIÓN CANGREJO - RETROCEDEMOS PARA AVANZAR"
echo "=============================================="

# 1. Limpiar configuración problemática
echo "🧹 Limpiando configuración Android..."
rm -f rust-toolchain.toml

# 2. Configurar Rust para Linux normal
echo "🔧 Configurando Rust para Linux..."
rustup default stable
rustup target add x86_64-unknown-linux-gnu

# 3. Crear configuración simple
echo "📝 Creando toolchain simple..."
cat > rust-toolchain.toml << 'TOOLCHAIN'
[toolchain]
channel = "stable"
TOOLCHAIN

# 4. Verificar configuración
echo "🔍 Verificando configuración actual:"
rustup show

# 5. Compilar de forma simple
echo "🔄 Compilando con configuración limpia..."
cargo clean
cargo build

# 6. Verificar resultado
if [ -f "./target/debug/mechbot-3x" ]; then
    echo "🎉 ¡FUNCIONÓ! MECHBOT-3X COMPILADO"
    echo "🚀 EJECUTANDO..."
    echo "================"
    ./target/debug/mechbot-3x
else
    echo "❌ Aún no funciona..."
    echo "💡 Probemos compilación mínima:"
    cargo check
fi
