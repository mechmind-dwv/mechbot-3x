#!/bin/bash

echo "🚀 ASISTENTE DE DESPLIEGUE MECHBOT-3X v2"
echo "=========================================="

# Verificar sistema
echo "🔍 Detectando sistema..."
if [ -f "/etc/os-release" ]; then
    source /etc/os-release
    echo "✅ Sistema: $PRETTY_NAME"
else
    echo "⚠️  Sistema no identificado"
fi

# Función para mostrar menú
show_menu() {
    echo ""
    echo "🎯 OPCIONES DE DESPLIEGUE:"
    echo "1) Reparar y compilar"
    echo "2) Desarrollo local"
    echo "3) Producción (systemd service)" 
    echo "4) Docker"
    echo "5) Solo compilación release"
    echo "6) Salir"
    echo ""
}

while true; do
    show_menu
    read -p "Selecciona opción (1-6): " deploy_option

    case $deploy_option in
        1)
            echo "🛠️  Reparando proyecto..."
            ./scripts/build_fix.sh
            ;;
            
        2)
            echo "💻 Configurando desarrollo local..."
            ./scripts/config/setup_interactive.sh
            ;;
            
        3)
            echo "🏗️  Configurando para producción..."
            
            # Primero compilar
            cargo build --release
            if [ $? -ne 0 ]; then
                echo "❌ Error en compilación. Usa opción 1 para reparar."
                continue
            fi
            
            # Crear servicio systemd
            sudo tee /etc/systemd/system/mechbot.service > /dev/null << SERVICE
[Unit]
Description=MechBot-3x Autonomous Robot
After=network.target

[Service]
Type=simple
User=$USER
WorkingDirectory=$PWD
ExecStart=$PWD/target/release/mechbot-3x
Restart=always
RestartSec=5
Environment=RUST_LOG=info

[Install]
WantedBy=multi-user.target
SERVICE

            sudo systemctl daemon-reload
            sudo systemctl enable mechbot
            sudo systemctl start mechbot
            
            echo "✅ Servicio instalado y iniciado"
            echo "📊 Comandos útiles:"
            echo "   sudo systemctl status mechbot"
            echo "   sudo journalctl -u mechbot -f"
            ;;
            
        4)
            echo "🐳 Configurando Docker..."
            
            # Verificar Docker
            if ! command -v docker &> /dev/null; then
                echo "❌ Docker no encontrado. Instalando..."
                curl -fsSL https://get.docker.com -o get-docker.sh
                sudo sh get-docker.sh
                sudo usermod -aG docker $USER
                echo "✅ Docker instalado. Reinicia sesión para usar Docker sin sudo."
            else
                echo "✅ Docker encontrado: $(docker --version)"
            fi
            
            # Build y run
            docker build -t mechbot-3x .
            docker run -d -p 8080:8080 -p 8081:8081 --name mechbot mechbot-3x
            
            echo "✅ Contenedor Docker ejecutándose"
            echo "📊 Ver logs: docker logs mechbot"
            ;;
            
        5)
            echo "🔨 Compilando para release..."
            cargo build --release
            if [ $? -eq 0 ]; then
                echo "✅ Compilación release exitosa"
                echo "🚀 Ejecuta: ./target/release/mechbot-3x"
            else
                echo "❌ Error en compilación release"
            fi
            ;;
            
        6)
            echo "👋 ¡Hasta pronto!"
            exit 0
            ;;
            
        *)
            echo "❌ Opción inválida. Por favor selecciona 1-6."
            ;;
    esac
    
    read -p "¿Continuar con otra operación? (s/n): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Ss]$ ]]; then
        echo "👋 ¡Hasta pronto!"
        break
    fi
done
