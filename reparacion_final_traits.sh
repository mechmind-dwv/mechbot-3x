#!/bin/bash
echo "🎯 REPARACIÓN FINAL - TRAITS MECHBOT-3X"

# 1. Agregar importación del trait Sensor
echo "🔧 Agregando importación del trait Sensor..."
sed -i '1s/^/use mechbot_3x::Sensor;\n/' src/main.rs

# 2. Verificar cambios
echo "✅ Importación agregada:"
head -5 src/main.rs

# 3. Compilar
echo "🔄 Compilando..."
cargo build --release

# 4. Ejecutar si compila
if [ -f "./target/release/mechbot-3x" ]; then
    echo "🚀 EJECUTANDO MECHBOT-3X:"
    ./target/release/mechbot-3x
else
    echo "❌ La compilación falló. Revisa los errores arriba."
fi
