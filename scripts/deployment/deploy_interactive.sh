#!/bin/bash

echo "🚀 ASISTENTE DE DESPLIEGUE MECHBOT-3X"
echo "======================================"

# Verificar sistema
echo "🔍 Detectando sistema..."
if [ -f "/etc/os-release" ]; then
    source /etc/os-release
    echo "✅ Sistema: $PRETTY_NAME"
else
    echo "⚠️  Sistema no identificado"
fi

# Opciones de despliegue
echo ""
echo "🎯 OPCIONES DE DESPLIEGUE:"
echo "1) Desarrollo local"
echo "2) Producción (systemd service)"
echo "3) Docker"
echo "4) Solo compilación"

read -p "Selecciona opción (1-4): " deploy_option

case $deploy_option in
    1)
        echo "🛠️  Configurando entorno de desarrollo..."
        ./scripts/config/setup_interactive.sh
        ;;
        
    2)
        echo "🏗️  Configurando para producción..."
        
        # Compilar
        cargo build --release
        
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
        
    3)
        echo "🐳 Configurando Docker..."
        
        # Crear Dockerfile si no existe
        if [ ! -f "Dockerfile" ]; then
            cat > Dockerfile << DOCKERFILE
FROM ubuntu:20.04

RUN apt-get update && apt-get install -y \\
    build-essential \\
    curl && \\
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV PATH="/root/.cargo/bin:\$PATH"

WORKDIR /app
COPY . .

RUN cargo build --release

CMD ["./target/release/mechbot-3x"]
DOCKERFILE
        fi
        
        # Build y run
        docker build -t mechbot-3x .
        docker run -d -p 8080:8080 -p 8081:8081 --name mechbot mechbot-3x
        
        echo "✅ Contenedor Docker ejecutándose"
        echo "📊 Ver logs: docker logs mechbot"
        ;;
        
    4)
        echo "🔨 Compilando proyecto..."
        cargo build --release
        if [ $? -eq 0 ]; then
            echo "✅ Compilación exitosa"
            echo "🚀 Ejecuta: ./target/release/mechbot-3x"
        else
            echo "❌ Error en compilación"
        fi
        ;;
        
    *)
        echo "❌ Opción inválida"
        exit 1
        ;;
esac

echo ""
echo "🎉 ¡Despliegue completado!"
