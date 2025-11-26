#!/bin/bash
echo "🌌 LANZAMIENTO CÓSMICO DEFINITIVO MECHBOT-3X"
echo "============================================"

# 1. Eliminar importación duplicada
echo "🔧 Eliminando importación duplicada..."
sed -i '2d' src/main.rs

# 2. Verificar que quedó solo una
echo "✅ Importaciones limpias:"
head -5 src/main.rs

# 3. Compilar versión final
echo "🔄 Compilando versión cósmica..."
cargo build --release

# 4. Ejecutar si compila
if [ -f "./target/release/mechbot-3x" ]; then
    echo ""
    echo "🎊 ¡MECHBOT-3X COMPILADO EXITOSAMENTE!"
    echo "🚀 INICIANDO SISTEMA AUTÓNOMO CÓSMICO..."
    echo "=========================================="
    ./target/release/mechbot-3x
else
    echo "❌ Error en compilación final."
    echo "💡 Última verificación con: cargo check"
    cargo check
fi
