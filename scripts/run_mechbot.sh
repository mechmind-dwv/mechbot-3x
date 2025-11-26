#!/bin/bash

echo "🚀 EJECUTANDO MECHBOT-3X"
echo "========================"

# Buscar el binario
BINARY=$(find target/ -name "mechbot-3x" -type f 2>/dev/null | head -1)

if [ -z "$BINARY" ]; then
    echo "❌ No se encontró el binario. Compilando..."
    cargo build --release
    BINARY=$(find target/ -name "mechbot-3x" -type f 2>/dev/null | head -1)
fi

if [ -n "$BINARY" ]; then
    echo "✅ Binario encontrado: $BINARY"
    echo "🔧 Versión:"
    $BINARY --version || echo "⚠️  No tiene flag --version"
    
    echo ""
    echo "🎯 Iniciando MechBot-3x..."
    echo "📊 Configuración:"
    echo "   - API REST: http://localhost:8088"
    echo "   - WebSocket: ws://localhost:8089" 
    echo "   - Logs: logs/mechbot.log"
    echo ""
    
    # Ejecutar
    $BINARY
else
    echo "❌ Error: No se pudo encontrar o compilar el binario"
    exit 1
fi
