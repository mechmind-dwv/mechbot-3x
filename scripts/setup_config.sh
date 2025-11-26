#!/bin/bash

# Colores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🔧 Configuración Interactiva - MechBot-3x${NC}"
echo "=========================================="

# Función para preguntas sí/no
ask_yes_no() {
    while true; do
        read -p "$1 [y/N]: " yn
        case $yn in
            [Yy]* ) return 0;;
            [Nn]* ) return 1;;
            * ) return 1;;
        esac
    done
}

# Función para input seguro
ask_password() {
    read -sp "$1: " password
    echo
    echo "$password"
}

# Verificar si config.toml existe
if [ -f "config.toml" ]; then
    echo -e "${YELLOW}⚠️  config.toml ya existe${NC}"
    if ask_yes_no "¿Quieres hacer backup y crear uno nuevo?"; then
        cp config.toml "config.backup.$(date +%Y%m%d_%H%M%S).toml"
        echo -e "${GREEN}✅ Backup creado${NC}"
    else
        echo -e "${YELLOW}🚫 Usando configuración existente${NC}"
        exit 0
    fi
fi

# Configuración interactiva
echo ""
echo -e "${BLUE}🤖 Configuración del Robot${NC}"
read -p "Nombre del robot: " robot_name
robot_name=${robot_name:-"MechBot-3x-001"}

read -p "Modelo: " robot_model
robot_model=${robot_model:-"MB3X"}

echo ""
echo -e "${BLUE}🌐 Configuración de API${NC}"
read -p "Puerto REST [8080]: " rest_port
rest_port=${rest_port:-"8080"}

read -p "Puerto WebSocket [8081]: " ws_port
ws_port=${ws_port:-"8081"}

if ask_yes_no "¿Requiere API Key?"; then
    api_key_required="true"
    api_key=$(openssl rand -base64 32)
    echo -e "${GREEN}🔑 API Key generada: ${api_key}${NC}"
    echo -e "${YELLOW}⚠️  ¡Guarda esta API Key en un lugar seguro!${NC}"
else
    api_key_required="false"
    api_key=""
fi

echo ""
echo -e "${BLUE}📡 Configuración de Sensores${NC}"
read -p "Puerto LIDAR [/dev/ttyUSB0]: " lidar_port
lidar_port=${lidar_port:-"/dev/ttyUSB0"}

read -p "Cámara index [0]: " camera_index
camera_index=${camera_index:-"0"}

# Crear archivo de configuración
cat > config.toml << CONFIG
[robot]
name = "$robot_name"
model = "$robot_model"
version = "3.0.0"

[api]
rest_port = $rest_port
websocket_port = $ws_port
enable_cors = true
api_key_required = $api_key_required

# Credenciales (solo si se generó API Key)
$([ ! -z "$api_key" ] && echo "api_key = \"$api_key\"")

[sensors]
lidar_port = "$lidar_port"
lidar_baudrate = 115200
camera_index = $camera_index
camera_fps = 30
imu_i2c_address = 0x68

[navigation]
max_speed = 2.0
max_acceleration = 1.0
planning_frequency = 10
obstacle_distance_threshold = 0.5

[logging]
level = "info"
output = "logs/mechbot.log"
max_file_size = "10MB"
rotate = true

[security]
# Configuración de seguridad adicional
session_timeout_minutes = 30
max_connections = 100
CONFIG

echo ""
echo -e "${GREEN}✅ Configuración guardada en config.toml${NC}"

# Crear directorio de logs
mkdir -p logs

# Preguntar por variables de entorno
if ask_yes_no "¿Quieres crear archivo .env para desarrollo?"; then
    cat > .env << ENV
# Configuración de desarrollo MechBot-3x
RUST_LOG=debug
MECHBOT_CONFIG=config.toml
MECHBOT_API_KEY=$api_key
MECHBOT_ENV=development

# Database (si se usa en el futuro)
# DB_HOST=localhost
# DB_PORT=5432
ENV
    echo -e "${GREEN}✅ Archivo .env creado${NC}"
    echo -e "${YELLOW}⚠️  Asegúrate de agregar .env a .gitignore${NC}"
fi

# Preguntar por compilación
if ask_yes_no "¿Compilar el proyecto ahora?"; then
    echo -e "${BLUE}🔨 Compilando...${NC}"
    cargo build --release
    
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✅ Compilación exitosa${NC}"
        echo -e "${BLUE}🤖 Ejecutar con: ./target/release/mechbot-3x${NC}"
    else
        echo -e "${RED}❌ Error en compilación${NC}"
    fi
fi

echo ""
echo -e "${GREEN}🎉 Configuración completada!${NC}"
echo -e "Archivos creados:"
echo -e "  📄 config.toml - Configuración principal"
[ -f ".env" ] && echo -e "  🔐 .env - Variables de entorno"
echo ""
echo -e "Siguientes pasos:"
echo -e "  1. Revisar config.toml"
echo -e "  2. Conectar hardware"
echo -e "  3. Ejecutar: ./target/release/mechbot-3x"
echo -e "  4. Verificar: curl http://localhost:$rest_port/api/v1/status"
