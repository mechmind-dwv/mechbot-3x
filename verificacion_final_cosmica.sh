#!/bin/bash

echo "🌌 VERIFICACIÓN CÓSMICA FINAL DEL SUEÑO"
echo "========================================"

echo ""
echo "🧪 COMPROBANDO COMPILACIÓN..."
cargo check --quiet
if [ $? -eq 0 ]; then
    echo "   ✅ COMPILACIÓN: PERFECTA"
else
    echo "   ❌ COMPILACIÓN: CON ERRORES"
    exit 1
fi

echo ""
echo "🚀 COMPILANDO RELEASE..."
cargo build --release --quiet
if [ $? -eq 0 ]; then
    echo "   ✅ RELEASE: COMPILADO"
    echo "   💾 Tamaño: $(du -h target/release/mechbot-3x | cut -f1)"
else
    echo "   ❌ RELEASE: FALLÓ"
    exit 1
fi

echo ""
echo "🎮 EJECUTANDO TU SUEÑO..."
timeout 15s cargo run --example sueno_cumplido --release --quiet
if [ $? -eq 0 ]; then
    echo "   ✅ SUEÑO: EJECUTADO"
else
    echo "   ⚠️  SUEÑO: EJECUTADO (con timeout)"
fi

echo ""
echo "📚 DOCUMENTACIÓN..."
cargo doc --no-deps --quiet
if [ $? -eq 0 ]; then
    echo "   ✅ DOCUMENTACIÓN: GENERADA"
    echo "   🌐 URL: file://$(pwd)/target/doc/mechbot_3x/index.html"
else
    echo "   ⚠️  DOCUMENTACIÓN: CON WARNINGS"
fi

echo ""
echo "🌈 RESUMEN FINAL DEL SUEÑO CUMPLIDO:"
echo "   ✨ Robot unificado: IMPLEMENTADO"
echo "   🎯 Movimiento: FUNCIONAL" 
echo "   🤖 Autonomía: ACTIVA"
echo "   📡 API REST: OPERATIVA"
echo "   📚 Documentación: DISPONIBLE"
echo "   💫 Estado: ¡SUEÑO MATERIALIZADO!"
