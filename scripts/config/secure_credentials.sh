#!/bin/bash

echo "🔐 CONFIGURACIÓN DE CREDENCIALES SEGURAS"
echo "========================================"

# Verificar si existe .env
if [ ! -f ".env" ]; then
    echo "📝 Creando archivo .env..."
    cat > .env << ENV_EXAMPLE
# Credenciales de MechBot-3x
MECHBOT_API_KEY=tu_api_key_secreta_aqui
MECHBOT_ADMIN_PASSWORD=password_seguro
DATABASE_URL=sqlite:///mechbot.db
LOG_LEVEL=info

# Configuración de red
API_HOST=0.0.0.0
API_PORT=8080
WS_PORT=8081

# Configuración de sensores
LIDAR_DEVICE=/dev/ttyUSB0
CAMERA_INDEX=0
ENV_EXAMPLE
    echo "✅ Archivo .env creado. Edita con tus credenciales reales."
else
    echo "✅ Archivo .env ya existe."
fi

# Generar API key segura si no existe
if ! grep -q "MECHBOT_API_KEY=" .env || grep -q "tu_api_key_secreta_aqui" .env; then
    echo "🔑 Generando nueva API key..."
    new_api_key=$(openssl rand -base64 32 2>/dev/null || date +%s | sha256sum | base64 | head -c 32)
    
    # Actualizar .env
    if [ -f ".env" ]; then
        sed -i "s/MECHBOT_API_KEY=.*/MECHBOT_API_KEY=$new_api_key/" .env
        echo "✅ Nueva API key generada: $new_api_key"
    fi
fi

# Configurar permisos seguros
chmod 600 .env 2>/dev/null || true

echo ""
echo "📋 INSTRUCCIONES:"
echo "1. Edita .env con tus credenciales reales"
echo "2. Nunca commits .env a Git"
echo "3. Usa variables de entorno en producción"
echo ""
echo "🚀 Para usar las credenciales:"
echo "   source .env && ./target/release/mechbot-3x"
