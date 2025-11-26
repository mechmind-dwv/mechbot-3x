#!/bin/bash

echo "🚀 PUSH MANUAL A REPOSITORIO REMOTO"
echo "==================================="

# Verificar estado remoto
echo "🔍 Verificando repositorio remoto..."
git remote -v

if [ $? -ne 0 ] || [ -z "$(git remote -v)" ]; then
    echo "❌ No hay repositorio remoto configurado"
    echo "📝 Para configurar:"
    echo "   git remote add origin https://github.com/usuario/repo.git"
    echo "   git push -u origin main"
    exit 1
fi

# Mostrar commits pendientes
echo "📊 Commits locales:"
git log --oneline -5

# Hacer push
echo "🚀 Haciendo push..."
git push origin feature/nueva-funcionalidad

if [ $? -eq 0 ]; then
    echo "✅ Push completado exitosamente"
    echo "🌐 URL del repositorio:"
    git remote get-url origin
else
    echo "❌ Error en push"
    echo "💡 Intentar: git push --set-upstream origin feature/nueva-funcionalidad"
fi
