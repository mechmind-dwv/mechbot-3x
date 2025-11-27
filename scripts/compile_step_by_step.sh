#!/bin/bash

echo "🏗️  Compilación paso a paso..."

# Paso 1: Solo verificar sintaxis
echo "1️⃣  Verificando sintaxis básica..."
cargo check --lib 2>&1 | grep -E "error\[|warning" | head -10

# Paso 2: Compilar módulos individualmente
echo "2️⃣  Compilando módulos individuales..."
for module in api config control navigation sensors vision; do
    echo "   📦 Compilando $module..."
    cargo check --lib --features "" 2>&1 | grep -E "error.*$module" | head -5 || true
done

# Paso 3: Verificar dependencias
echo "3️⃣  Verificando dependencias..."
cargo tree --depth 1

# Paso 4: Compilación final
echo "4️⃣  Compilación final..."
if cargo build --release; then
    echo "🎉 ¡Compilación exitosa!"
    ls -la target/release/mechbot-3x 2>/dev/null && echo "✅ Binario creado correctamente"
else
    echo "❌ Compilación fallida"
    # Mostrar últimos errores
    cargo check 2>&1 | tail -20
fi
