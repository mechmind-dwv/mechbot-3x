#!/bin/bash

echo "🚀 ASISTENTE DE DESPLIEGUE CORREGIDO"
echo "===================================="

# Verificar target correcto
echo "🔍 Verificando target..."
CURRENT_TARGET=$(rustc -vV 2>/dev/null | grep host | cut -d' ' -f2 || echo "unknown")

if [[ "$CURRENT_TARGET" == *"android"* ]]; then
    echo "❌ Target incorrecto: $CURRENT_TARGET"
    echo "🔄 Ejecutando reparación..."
    ./scripts/fix_android_target.sh
    exit 0
fi

echo "✅ Target correcto: $CURRENT_TARGET"

# Menú principal
while true; do
    echo ""
    echo "🎯 OPCIONES:"
    echo "1) Compilar para Linux"
    echo "2) Desarrollo interactivo" 
    echo "3) Producción"
    echo "4) Verificar sistema"
    echo "5) Salir"
    echo ""
    
    read -p "Selecciona opción (1-5): " option
    
    case $option in
        1)
            ./scripts/compile_linux.sh
            ;;
        2)
            ./scripts/config/setup_interactive.sh
            ;;
        3)
            echo "🏗️ Configurando producción..."
            cargo build --release
            if [ $? -eq 0 ]; then
                echo "✅ Release compilado: ./target/release/mechbot-3x"
            fi
            ;;
        4)
            echo "🔍 DIAGNÓSTICO DEL SISTEMA:"
            echo "Rust: $(rustc --version 2>/dev/null || echo 'No encontrado')"
            echo "Target: $(rustc -vV 2>/dev/null | grep host | cut -d' ' -f2 || echo 'Unknown')"
            echo "Sistema: $(uname -m)-$(uname -s)"
            echo "Directorio: $(pwd)"
            find target/ -name "mechbot-3x" -type f 2>/dev/null | head -5
            ;;
        5)
            echo "👋 ¡Hasta pronto!"
            exit 0
            ;;
        *)
            echo "❌ Opción inválida"
            ;;
    esac
done
