#!/bin/bash

echo "🔍 DIAGNÓSTICO COMPLETO DEL BINARIO"
echo "==================================="

# 1. Verificar compilación
echo "1. Verificando compilación..."
cargo build --release --message-format=json | grep -i "executable\|binario" || echo "✅ Compilación completada"

# 2. Buscar ejecutables
echo ""
echo "2. Buscando ejecutables..."
EXECUTABLES=$(find target/ -type f -executable 2>/dev/null | grep -v "\.d\|\.so\|\.rlib")
if [ -n "$EXECUTABLES" ]; then
    echo "📁 Ejecutables encontrados:"
    echo "$EXECUTABLES"
    for exe in $EXECUTABLES; do
        echo "   📊 $exe: $(file "$exe" 2>/dev/null || echo "desconocido")"
    done
else
    echo "❌ No se encontraron ejecutables"
fi

# 3. Ver estructura de target
echo ""
echo "3. Estructura de target/:"
if command -v tree &> /dev/null; then
    tree target/ || find target/ -type d | head -20
else
    find target/ -type d | head -20
fi

# 4. Verificar Cargo.toml
echo ""
echo "4. Verificando Cargo.toml..."
grep -A5 -B5 "\[\[bin\]\]" Cargo.toml || echo "⚠️  No se encontró configuración [[bin]] explícita"

# 5. Verificar nombre del paquete
echo ""
echo "5. Nombre del paquete:"
cargo metadata --format-version=1 | jq -r '.packages[0].targets[] | select(.kind[0] == "bin") | .name' 2>/dev/null || echo "❌ No se pudo obtener nombre"

# 6. Intentar compilar con output verbose
echo ""
echo "6. Compilación verbose..."
cargo build --release --verbose 2>&1 | grep -i "linking\|executable\|/mechbot" | tail -5
