#!/bin/bash
echo "🚀 LANZAMIENTO FINAL MECHBOT-3X"
echo "================================"

# 1. Hacer api_server mutable
echo "🔧 Haciendo api_server mutable..."
sed -i 's/Ok((lidar, camera, imu, navigation, vision, api_server))/Ok((lidar, camera, imu, navigation, vision, mut api_server))/g' src/main.rs

# 2. Verificar cambio
echo "✅ Cambio aplicado:"
grep -n "mut api_server" src/main.rs

# 3. Compilar en modo release
echo "🔄 Compilando versión final..."
cargo build --release

# 4. Ejecutar si compila
if [ -f "./target/release/mechbot-3x" ]; then
    echo ""
    echo "🎉 ¡MECHBOT-3X COMPILADO EXITOSAMENTE!"
    echo "🤖 INICIANDO SISTEMA AUTÓNOMO..."
    echo "========================================"
    ./target/release/mechbot-3x
else
    echo "❌ Error en compilación final."
    echo "💡 Ejecuta 'cargo check' para ver errores detallados."
fi
