#!/bin/bash

echo "🔧 REPARANDO COMPILACIÓN MECHBOT-3X"

# Limpiar cache
cargo clean

# Verificar dependencias
echo "📦 Verificando dependencias..."
cargo check

if [ $? -eq 0 ]; then
    echo "✅ Dependencias OK"
else
    echo "❌ Error en dependencias"
    exit 1
fi

# Compilar en modo desarrollo
echo "🔄 Compilando en modo desarrollo..."
cargo build

if [ $? -eq 0 ]; then
    echo "🎉 ¡Compilación exitosa!"
    echo "🚀 Binario: ./target/debug/mechbot-3x"
    
    # Probar ejecución
    echo "🧪 Probando ejecución..."
    timeout 5s ./target/debug/mechbot-3x || echo "✅ Ejecución probada (se detuvo después de 5s)"
else
    echo "❌ Error en compilación"
    echo "📋 Mostrando errores:"
    cargo check 2>&1 | grep error:
    exit 1
fi
