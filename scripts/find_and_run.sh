#!/bin/bash

echo "🎯 BUSCANDO Y EJECUTANDO MECHBOT-3X"
echo "==================================="

# Método 1: Buscar en target/
echo "1. Buscando en target/..."
BINARY1=$(find target/ -type f -executable -name "mechbot-3x" 2>/dev/null | head -1)

# Método 2: Buscar cualquier binario reciente
echo "2. Buscando binarios recientes..."
BINARY2=$(find target/ -type f -executable -mmin -5 2>/dev/null | head -1)

# Método 3: Buscar por nombre de paquete
echo "3. Buscando por nombre de paquete..."
BINARY3=$(find target/ -type f -executable -name "*mechbot*" 2>/dev/null | head -1)

# Combinar resultados
BINARY="$BINARY1"
[ -z "$BINARY" ] && BINARY="$BINARY2"
[ -z "$BINARY" ] && BINARY="$BINARY3"

if [ -n "$BINARY" ]; then
    echo "🎉 BINARIO ENCONTRADO: $BINARY"
    echo "📊 Información:"
    file "$BINARY"
    echo ""
    echo "🚀 EJECUTANDO..."
    "$BINARY"
else
    echo "❌ No se encontró el binario"
    echo ""
    echo "🔧 SOLUCIONES:"
    echo "1. Verificar que Cargo.toml tiene [[bin]] name = 'mechbot-3x'"
    echo "2. Ejecutar: cargo clean && cargo build --release --verbose"
    echo "3. Revisar mensajes de compilación para ver dónde se guarda el binario"
    
    # Compilar con output verbose
    echo ""
    echo "🔨 Compilando con output verbose..."
    cargo build --release --verbose 2>&1 | grep -i "linking\|target.*release\|executable"
fi
