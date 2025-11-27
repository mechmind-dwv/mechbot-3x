#!/bin/bash

echo "🔍 Diagnóstico detallado de errores..."

# 1. Mostrar errores específicos
echo "📋 Errores de compilación:"
cargo check 2>&1 | grep -E "(error\[E[0-9]+\]|warning)" | head -30

# 2. Analizar tipos de errores
echo "📊 Resumen de errores:"
cargo check 2>&1 | grep -o "error\[E[0-9]+\]" | sort | uniq -c | sort -nr

# 3. Verificar imports problemáticos
echo "📥 Imports problemáticos:"
find src -name "*.rs" -exec grep -l "use.*::.*;" {} \; | xargs -I {} grep -n "use.*::" {} | grep -v "//" | head -10

# 4. Verificar traits conflictivos
echo "🔄 Traits conflictivos:"
find src -name "*.rs" -exec grep -l "impl.*for" {} \; | xargs -I {} grep -n "impl.*for" {} | head -10

# 5. Verificar estructuras duplicadas
echo "🏗️  Estructuras duplicadas:"
find src -name "*.rs" -exec grep -h "^pub struct\|^struct" {} \; | sort | uniq -d

# 6. Verificar módulos
echo "📦 Módulos problemáticos:"
find src -name "mod.rs" | head -5
