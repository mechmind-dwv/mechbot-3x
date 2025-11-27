#!/bin/bash

echo "🔧 Iniciando reparación de errores de compilación..."

# 1. Limpiar proyecto
echo "🧹 Limpiando build..."
cargo clean

# 2. Verificar estructura de archivos
echo "📁 Verificando estructura..."
find src -name "*.rs" | head -10

# 3. Arreglar errores comunes paso a paso
echo "🔍 Analizando errores..."

# Arreglar warning de variable no usada
echo "📝 Corrigiendo variables no usadas..."
find src -name "*.rs" -exec sed -i 's/current_pose: &RobotState/_current_pose: \&RobotState/g' {} \;
find src -name "*.rs" -exec sed -i 's/current_pose/_current_pose/g' {} \;

# 4. Verificar compilación básica
echo "🛠️  Intentando compilación básica..."
cargo check 2>&1 | head -20

# 5. Si hay errores específicos, aplicar fixes
if cargo check 2>&1 | grep -q "E0034"; then
    echo "⚠️  Encontrado error E0034 (método ambiguo)..."
    # Buscar y mostrar métodos problemáticos
    find src -name "*.rs" -exec grep -l "impl.*for.*{" {} \; | head -5
fi

# 6. Instalar herramientas necesarias
echo "📦 Instalando herramientas..."
cargo install cargo-watch 2>/dev/null || echo "cargo-watch ya instalado"

# 7. Ejecutar formateo
echo "🎨 Aplicando formato..."
cargo fmt

# 8. Verificar estado final
echo "✅ Verificación final..."
cargo check --quiet && echo "🎉 ¡Compilación exitosa!" || echo "❌ Aún hay errores por resolver"

echo "📋 Resumen:"
echo "   - Proyecto limpiado"
echo "   - Variables no usadas corregidas" 
echo "   - Formato aplicado"
echo "   - Estado de compilación verificado"
