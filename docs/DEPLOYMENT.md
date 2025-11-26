# 🚀 Guía de Despliegue en Producción - MechBot-3x

Esta guía cubre el despliegue de MechBot-3x en entornos de producción.

## 📋 Prerrequisitos de Producción

### Requisitos de Hardware
- **CPU**: 4+ cores (8+ recomendado)
- **RAM**: 8GB mínimo, 16GB recomendado
- **Almacenamiento**: 50GB SSD
- **Red**: Ethernet estable (WiFi opcional)
- **UPS**: Para operación continua

### Requisitos de Software
- **OS**: Ubuntu Server 20.04 LTS o superior
- **Kernel**: 5.4+ para mejor soporte de hardware
- **Docker**: 20.10+ (opcional, para contenedores)

## 🏗️ Estrategias de Despliegue

### Opción 1: Despliegue Directo (Recomendado)

#### 1. Preparar el Sistema
```bash
# Actualizar sistema
sudo apt update && sudo apt upgrade -y

# Instalar dependencias
sudo apt install -y \
    build-essential \
    cmake \
    pkg-config \
    libopencv-dev \
    libusb-1.0-0-dev \
    libssl-dev \
    curl

# Instalar Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source ~/.cargo/env
```

#### 2. Desplegar la Aplicación
```bash
# Clonar o transferir el código
git clone https://github.com/mechmind-dwv/mechbot-3x.git
cd mechbot-3x

# Compilar en modo release
cargo build --release

# Verificar la build
./target/release/mechbot-3x --version
```

#### 3. Configurar como Servicio Systemd
```bash
sudo cat > /etc/systemd/system/mechbot.service << 'SERVICE'
[Unit]
Description=MechBot-3x Autonomous Robot
After=network.target
Wants=network.target

[Service]
Type=simple
User=mechbot
Group=mechbot
WorkingDirectory=/opt/mechbot-3x
ExecStart=/opt/mechbot-3x/target/release/mechbot-3x
Restart=always
RestartSec=5
Environment=RUST_LOG=info

# Security
NoNewPrivileges=yes
PrivateTmp=yes
ProtectSystem=strict
ProtectHome=yes
ReadWritePaths=/opt/mechbot-3x/logs

# Resource limits
MemoryLimit=2G
CPUQuota=80%

[Install]
WantedBy=multi-user.target
SERVICE

# Crear usuario dedicado
sudo useradd -r -s /bin/false mechbot
sudo chown -R mechbot:mechbot /opt/mechbot-3x

# Habilitar y iniciar servicio
sudo systemctl daemon-reload
sudo systemctl enable mechbot
sudo systemctl start mechbot
```

### Opción 2: Despliegue con Docker

#### Dockerfile de Producción
```dockerfile
FROM ubuntu:20.04

# Instalar dependencias del sistema
RUN apt-get update && apt-get install -y \
    build-essential \
    cmake \
    pkg-config \
    libopencv-dev \
    libusb-1.0-0-dev \
    libssl-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Instalar Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Copiar código y compilar
WORKDIR /app
COPY . .
RUN cargo build --release

# Configurar runtime
USER 1000:1000
EXPOSE 8080 8081
CMD ["./target/release/mechbot-3x"]
```

#### Docker Compose para Producción
```yaml
version: '3.8'

services:
  mechbot:
    build: .
    ports:
      - "8080:8080"
      - "8081:8081"
    volumes:
      - ./config.toml:/app/config.toml:ro
      - ./logs:/app/logs
    devices:
      - "/dev/ttyUSB0:/dev/ttyUSB0"
      - "/dev/video0:/dev/video0"
    restart: unless-stopped
    environment:
      - RUST_LOG=info
    deploy:
      resources:
        limits:
          memory: 2G
          cpus: '2.0'
```

## 🔧 Configuración de Producción

### Configuración Optimizada
```toml
# config.production.toml
[robot]
name = "mechbot-production-01"
model = "MB3X"
version = "3.0.0"

[sensors]
lidar_port = "/dev/ttyUSB0"
lidar_baudrate = 115200
camera_index = 0
camera_fps = 15  # Reducido para producción
imu_i2c_address = 0x68

[navigation]
max_speed = 1.5  # Más conservador
max_acceleration = 0.8
planning_frequency = 5  # Menos frecuente

[api]
rest_port = 8080
websocket_port = 8081
enable_cors = true
api_key_required = true
rate_limit_requests_per_minute = 600

[logging]
level = "info"
output = "file"
file_path = "/var/log/mechbot/mechbot.log"
max_file_size = "100MB"
rotate = true
max_backups = 10

[security]
allowed_ips = ["192.168.1.0/24", "10.0.0.0/8"]
session_timeout_minutes = 30

[monitoring]
health_check_interval = 30
metrics_enabled = true
```

### Configuración de Red y Seguridad
```bash
# Configurar firewall
sudo ufw enable
sudo ufw allow 22    # SSH
sudo ufw allow 8080  # API REST
sudo ufw allow 8081  # WebSocket

# Configurar HTTPS con reverse proxy (opcional)
sudo apt install nginx
sudo cat > /etc/nginx/sites-available/mechbot << 'NGINX'
server {
    listen 443 ssl;
    server_name mechbot.example.com;

    ssl_certificate /etc/ssl/certs/mechbot.crt;
    ssl_certificate_key /etc/ssl/private/mechbot.key;

    location / {
        proxy_pass http://localhost:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }

    location /ws {
        proxy_pass http://localhost:8081;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
    }
}
NGINX
```

## 📊 Monitoreo y Logging

### Configuración de Logging Avanzado
```toml
[logging]
level = "info"
output = "file"
file_path = "/var/log/mechbot/mechbot.log"
max_file_size = "100MB"
rotate = true
max_backups = 10
json_format = true  # Para ingestión en ELK

[monitoring]
prometheus_enabled = true
prometheus_port = 9090
health_check_interval = 30
```

### Script de Monitoreo
```bash
#!/bin/bash
# monitoring.sh

HEALTH_CHECK_URL="http://localhost:8080/api/v1/health"
ALERT_EMAIL="alerts@example.com"

check_health() {
    response=$(curl -s -o /dev/null -w "%{http_code}" $HEALTH_CHECK_URL)
    if [ "$response" -ne 200 ]; then
        echo "ALERT: MechBot health check failed" | mail -s "MechBot Alert" $ALERT_EMAIL
        return 1
    fi
    return 0
}

check_resources() {
    memory_usage=$(ps -o pid,ppid,pcpu,pmem,cmd -C mechbot-3x | awk 'NR==2 {print $4}')
    if (( $(echo "$memory_usage > 80.0" | bc -l) )); then
        echo "ALERT: High memory usage: ${memory_usage}%" | mail -s "MechBot Resource Alert" $ALERT_EMAIL
    fi
}

# Ejecutar checks cada minuto
while true; do
    check_health
    check_resources
    sleep 60
done
```

## 🔄 Actualizaciones y Mantenimiento

### Estrategia de Actualización
```bash
#!/bin/bash
# update_mechbot.sh

echo "🔄 Iniciando actualización de MechBot-3x..."

# Backup de configuración
cp config.toml config.toml.backup.$(date +%Y%m%d)

# Detener servicio
sudo systemctl stop mechbot

# Actualizar código
git pull origin main

# Recompilar
cargo build --release

# Verificar nueva versión
./target/release/mechbot-3x --version

# Restaurar configuración si es necesario
cp config.toml.backup.$(date +%Y%m%d) config.toml

# Iniciar servicio
sudo systemctl start mechbot

echo "✅ Actualización completada"
```

### Rollback Automático
```bash
#!/bin/bash
# rollback.sh

echo "🔄 Ejecutando rollback..."

sudo systemctl stop mechbot

# Restaurar backup anterior
cp config.toml.backup.$(date -d "yesterday" +%Y%m%d) config.toml
git checkout HEAD~1

cargo build --release
sudo systemctl start mechbot

echo "✅ Rollback completado"
```

## 🗂️ Estructura de Directorios de Producción

```
/opt/mechbot-3x/
├── target/release/mechbot-3x    # Binario principal
├── config.toml                  # Configuración
├── config.backups/              # Backups de configuración
├── logs/                        # Logs de aplicación
│   ├── mechbot.log
│   └── mechbot.log.1
├── data/                        # Datos persistentes
│   ├── maps/
│   └── models/
└── scripts/                     # Scripts de mantenimiento
    ├── update_mechbot.sh
    ├── rollback.sh
    └── monitoring.sh
```

## 🔐 Seguridad en Producción

### Hardening del Sistema
```bash
# Actualizaciones automáticas de seguridad
sudo apt install unattended-upgrades
sudo dpkg-reconfigure unattended-upgrades

# Configurar fail2ban para protección SSH
sudo apt install fail2ban

# Limitar acceso a puertos
sudo ufw default deny incoming
sudo ufw default allow outgoing
sudo ufw allow from 192.168.1.0/24 to any port 22
```

### Seguridad de la Aplicación
```toml
[security]
api_key_required = true
allowed_ips = ["192.168.1.0/24"]
session_timeout_minutes = 30
max_request_size = "10MB"
rate_limit_requests_per_minute = 600

[ssl]
enabled = true
cert_file = "/etc/ssl/certs/mechbot.crt"
key_file = "/etc/ssl/private/mechbot.key"
```

## 📈 Escalabilidad

### Configuración para Múltiples Robots
```bash
# Script para gestionar flota
#!/bin/bash
ROBOTS=("robot-01" "robot-02" "robot-03")

for robot in "${ROBOTS[@]}"; do
    echo "🔄 Actualizando $robot..."
    ssh mechbot@$robot "/opt/mechbot-3x/scripts/update_mechbot.sh"
done
```

### Load Balancer para API
```nginx
upstream mechbot_servers {
    server 192.168.1.101:8080;
    server 192.168.1.102:8080;
    server 192.168.1.103:8080;
}

server {
    listen 80;
    location / {
        proxy_pass http://mechbot_servers;
    }
}
```

## 🚨 Plan de Recuperación de Desastres

### Backup Automático
```bash
#!/bin/bash
# backup.sh

BACKUP_DIR="/backup/mechbot"
DATE=$(date +%Y%m%d_%H%M%S)

# Crear backup
tar -czf $BACKUP_DIR/mechbot_$DATE.tar.gz \
    /opt/mechbot-3x/config.toml \
    /opt/mechbot-3x/data/ \
    /var/log/mechbot/

# Rotar backups (mantener últimos 7 días)
find $BACKUP_DIR -name "mechbot_*.tar.gz" -mtime +7 -delete

echo "✅ Backup completado: mechbot_$DATE.tar.gz"
```

### Recovery Procedure
1. **Servicio caído**: `sudo systemctl restart mechbot`
2. **Configuración corrupta**: Restaurar desde backup
3. **Hardware fallado**: Reemplazar y restaurar configuración
4. **Data loss**: Restaurar desde último backup

---

**¡Despliegue exitoso!** 🎉 
Tu MechBot-3x ahora está funcionando en un entorno de producción robusto y monitoreado.
