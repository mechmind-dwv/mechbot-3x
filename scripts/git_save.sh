#!/bin/bash

echo "💾 GUARDANDO PROYECTO EN GIT"
echo "============================"

# Verificar estado
echo "📊 Estado actual:"
git status --short

# Confirmar
read -p "¿Continuar con commit? (s/n): " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Ss]$ ]]; then
    echo "❌ Cancelado"
    exit 1
fi

# Agregar cambios
echo "📦 Agregando archivos..."
git add .

# Hacer commit
echo "💡 Creando commit..."
git commit -m "🔄 Update: $(date '+%Y-%m-%d %H:%M:%S')

- Actualizaciones de código
- Mejoras en documentación
- Scripts de desarrollo
- Configuración del proyecto"

# Mostrar resultado
echo "✅ Commit creado:"
git log --oneline -1

# Preguntar por push
read -p "¿Hacer push al repositorio remoto? (s/n): " -n 1 -r
echo
if [[ $REPLY =~ ^[Ss]$ ]]; then
    echo "🚀 Haciendo push..."
    git push origin main
    echo "✅ Push completado"
else
    echo "💡 Cambios guardados localmente. Usa 'git push' cuando quieras subirlos."
fi

echo ""
echo "🎉 ¡Proyecto guardado exitosamente!"
