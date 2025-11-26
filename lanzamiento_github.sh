#!/bin/bash
echo "🌌 PREPARANDO LANZAMIENTO GITHUB MECHBOT-3X"
echo "==========================================="

# 1. Verificar que tenemos los archivos esenciales
echo "📋 Verificando archivos esenciales..."
ESSENTIAL_FILES=("src/main.rs" "Cargo.toml" "README.md" "LICENSE" "config.toml")
for file in "${ESSENTIAL_FILES[@]}"; do
    if [ -f "$file" ]; then
        echo "✅ $file"
    else
        echo "❌ $file (FALTANTE)"
    fi
done

# 2. Agregar archivos al repositorio
echo "🔄 Agregando archivos a git..."
git add .

# 3. Hacer commit del lanzamiento cangrejo
echo "💾 Creando commit histórico..."
git commit -m "🚀 Lanzamiento MechBot-3X Cangrejo

✨ Características:
- Sistema autónomo basado en estrategia cangrejo
- Retroceso productivo implementado
- Navegación cósmica mejorada
- 100% Rust-powered

🎯 Logros:
- Binario funcional: mechbot-3x (3.9 MB)
- Misiones completadas: 3/3
- Errores superados: ∞
- Certificación cangrejo obtenida

🦀 Táctica probada: Avanzar retrocediendo"

# 4. Mostrar información del commit
echo "📊 Resumen del commit:"
git log --oneline -1

# 5. Instrucciones para conectar con GitHub
echo ""
echo "🎯 PASOS PARA SUBIR A GITHUB:"
echo "=============================="
echo "1. Crear repositorio en GitHub: https://github.com/new"
echo "2. Nombre: mechbot-3x"
echo "3. Descripción: 'Sistema de robótica autónoma de tercera generación con IA integrada'"
echo "4. No inicializar con README (ya tenemos uno)"
echo ""
echo "5. Conectar repositorio local:"
echo "   git remote add origin https://github.com/mechmind-dwv/mechbot-3x.git"
echo "   git branch -M main"
echo "   git push -u origin main"
echo ""
echo "6. 🎉 ¡REPOSITORIO PUBLICADO!"

# 6. Mostrar comando listo para copiar y pegar
echo ""
echo "📋 COMANDO LISTO (copia y pega después de crear el repo en GitHub):"
echo "git remote add origin https://github.com/mechmind-dwv/mechbot-3x.git && git branch -M main && git push -u origin main"
