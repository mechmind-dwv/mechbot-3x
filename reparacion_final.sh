#!/bin/bash
echo "🎯 REPARACIÓN FINAL MECHBOT-3X"

# 1. Corregir método get_position
sed -i 's/self\.navigation\.get_position()/self\.navigation\.get_current_position()/g' src/robot.rs

# 2. Verificar cambios
echo "✅ Método corregido:"
grep -n "get_current_position" src/robot.rs

# 3. Compilar
echo "🔄 Compilando..."
cargo build --release

# 4. Ejecutar
if [ -f "./target/release/mechbot-3x" ]; then
    echo "🚀 EJECUTANDO MECHBOT-3X:"
    ./target/release/mechbot-3x
else
    echo "❌ La compilación falló. Revisa los errores arriba."
fi
