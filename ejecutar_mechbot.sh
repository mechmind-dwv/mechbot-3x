#!/bin/bash
echo "🚀 EJECUTANDO MECHBOT-3X..."
echo "============================"

# Primero intentar con debug (que sabemos que existe)
if [ -f "./target/debug/mechbot-3x" ]; then
    echo "✅ Ejecutando versión DEBUG..."
    ./target/debug/mechbot-3x
elif [ -f "./target/release/mechbot-3x" ]; then
    echo "✅ Ejecutando versión RELEASE..."
    ./target/release/mechbot-3x
else
    echo "❌ No se encontró el binario. Compilando..."
    cargo build --release
    if [ -f "./target/release/mechbot-3x" ]; then
        echo "✅ Compilado exitoso. Ejecutando..."
        ./target/release/mechbot-3x
    else
        echo "❌ Error: No se pudo compilar el binario"
    fi
fi
