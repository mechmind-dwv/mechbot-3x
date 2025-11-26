# 🛠️ Guía de Solución de Problemas - MechBot-3x

Esta guía te ayudará a diagnosticar y resolver problemas comunes del sistema.

## 🔍 Diagnóstico Rápido

### Comando de Diagnóstico Automático
```bash
# Ejecutar diagnóstico completo
./target/release/mechbot-3x --diagnose

# O usar el script de verificación
chmod +x scripts/diagnose_system.sh
./scripts/diagnose_system.sh
```

### Verificar Estado del Sistema
```bash
# Verificar servicios en ejecución
ps aux | grep mechbot
systemctl status mechbot  # Si está como servicio

# Verificar uso de recursos
top -p $(pgrep mechbot)
df -h  # Espacio en disco
free -h  # Memoria
```

## 🚨 Problemas Comunes y Soluciones

### 1. ❌ "Error de Compilación"

#### Síntomas:
```
error[E0463]: can't find crate for `core`
error: linking with `cc` failed
```

#### Soluciones:
```bash
# Solución 1: Limpiar y recompilar
cargo clean
cargo build

# Solución 2: Reinstalar Rust
rustup self update
rustup update stable
rustup default stable

# Solución 3: Verificar dependencias del sistema
sudo apt update && sudo apt install build-essential
```

### 2. ❌ "No se puede encontrar /dev/ttyUSB0"

#### Síntomas:
```
Error: LIDAR device not found: /dev/ttyUSB0
```

#### Soluciones:
```bash
# Verificar dispositivos USB
lsusb
ls -la /dev/ttyUSB*

# Agregar usuario al grupo dialout
sudo usermod -a -G dialout $USER
# Cerrar sesión y volver a entrar

# Verificar permisos
sudo chmod 666 /dev/ttyUSB0

# Probar con otro puerto
# Modificar config.toml: port = "/dev/ttyUSB1"
```

### 3. ❌ "Error de Cámara: Cannot open /dev/video0"

#### Síntomas:
```
ERROR camera: Failed to open camera: Permission denied
```

#### Soluciones:
```bash
# Verificar cámaras disponibles
v4l2-ctl --list-devices

# Probar con VLC para verificar hardware
vlc v4l2:///dev/video0

# Solucionar permisos
sudo usermod -a -G video $USER
# Cerrar sesión y volver a entrar

# Probar con índice diferente
# En config.toml: camera_index = 1
```

### 4. ❌ "API no responde en puerto 8080"

#### Síntomas:
```
curl: (7) Failed to connect to localhost port 8080
```

#### Soluciones:
```bash
# Verificar si el proceso está ejecutándose
ps aux | grep mechbot

# Verificar puertos en uso
netstat -tulpn | grep 8080
sudo lsof -i :8080

# Matar proceso si está ocupado
sudo kill -9 $(lsof -t -i:8080)

# Verificar firewall
sudo ufw status
sudo ufw allow 8080

# Ejecutar con puerto diferente
./target/release/mechbot-3x --port 8082
```

### 5. ❌ "Error de Memoria o CPU al 100%"

#### Síntomas:
- Sistema lento
- Consumo alto de CPU/RAM
- El robot no responde

#### Soluciones:
```bash
# Identificar proceso problemático
htop
ps aux --sort=-%cpu | head -10

# Limitar recursos (si es necesario)
systemctl set-property mechbot.service CPUQuota=80%

# Optimizar configuración
# En config.toml reducir:
# - camera_fps = 15
# - lidar_scan_rate = 5
```

### 6. ❌ "IMU no detectado en dirección I2C"

#### Síntomas:
```
ERROR imu: I2C device not found at address 0x68
```

#### Soluciones:
```bash
# Verificar buses I2C
i2cdetect -l

# Escanear dispositivos I2C
i2cdetect -y 1  # Para Raspberry Pi

# Verificar conexiones físicas
# - Cableado SDA/SCL correcto
# - Alimentación 3.3V estable
# - Resistores pull-up

# Probar con otra dirección
# En config.toml: imu_i2c_address = 0x69
```

## 📊 Análisis de Logs

### Niveles de Log
```toml
[logging]
level = "debug"  # Para troubleshooting
output = "file"
```

### Comandos Útiles para Logs
```bash
# Ver logs en tiempo real
tail -f mechbot.log

# Buscar errores
grep -i "error" mechbot.log
grep -i "warn" mechbot.log

# Logs por módulo
grep "navigation" mechbot.log
grep "sensors" mechbot.log

# Análisis de tiempos
grep "processing_time" mechbot.log
```

### Interpretación de Logs Comunes

#### Log de Inicialización Exitosa:
```
INFO [mechbot] System initialized successfully
INFO [sensors] LIDAR connected: /dev/ttyUSB0
INFO [sensors] IMU calibrated successfully  
INFO [api] REST server started on :8080
```

#### Log con Problemas:
```
ERROR [sensors] LIDAR scan failed: timeout
WARN [navigation] Localization uncertainty high: 0.45
ERROR [vision] Object detection model load failed
```

## 🔧 Herramientas de Debugging

### Debug Mode
```bash
# Ejecutar en modo debug
RUST_LOG=debug ./target/debug/mechbot-3x

# O compilar con símbolos de debug
cargo build
./target/debug/mechbot-3x --debug
```

### Profiling de Performance
```bash
# Instalar herramientas de profiling
cargo install flamegraph

# Generar flamegraph
cargo flamegraph --bin mechbot-3x

# Profiling con perf
perf record ./target/release/mechbot-3x
perf report
```

### Monitoreo de Recursos
```bash
# Script de monitoreo
#!/bin/bash
while true; do
    echo "=== $(date) ==="
    ps -p $(pgrep mechbot) -o pid,ppid,pcpu,pmem,cmd
    echo "---"
    sleep 5
done
```

## 🐛 Bugs Conocidos y Workarounds

### 1. **Memory Leak en Procesamiento de Imágenes**
**Síntoma**: Uso de memoria creciente con el tiempo
**Workaround**: 
```toml
[vision]
max_cache_size = 100  # Limitar cache de frames
```

### 2. **Race Condition en Inicialización de Sensores**
**Síntoma**: Sensores no inicializan ocasionalmente
**Workaround**:
```bash
# Reiniciar servicio de sensores
systemctl restart mechbot-sensors
```

### 3. **Problema de Timing en LIDAR**
**Síntoma**: Datos LIDAR inconsistentes
**Workaround**:
```toml
[sensors.lidar]
timeout_ms = 5000  # Aumentar timeout
scan_rate = 5      # Reducir tasa de escaneo
```

## 📞 Soporte Técnico

### Información para Reportar Bugs
Cuando reportes un problema, incluye:

1. **Versión del sistema**: `./target/release/mechbot-3x --version`
2. **Logs completos**: `tail -n 100 mechbot.log`
3. **Configuración**: `cat config.toml` (sin passwords)
4. **Hardware**: `lscpu`, `lsusb`, `i2cdetect -y 1`
5. **Pasos para reproducir**: Comandos exactos ejecutados

### Canales de Soporte
- **GitHub Issues**: Para bugs y feature requests
- **Discord**: Para ayuda en tiempo real
- **Email**: Para consultas específicas

### Script de Recopilación de Diagnóstico
```bash
#!/bin/bash
# diagnosis_collector.sh
echo "📋 Recopilando información de diagnóstico..."

# Información del sistema
echo "=== SYSTEM INFO ===" > diagnosis.txt
uname -a >> diagnosis.txt
rustc --version >> diagnosis.txt

# Configuración
echo "=== CONFIG ===" >> diagnosis.txt
cat config.toml >> diagnosis.txt 2>/dev/null || echo "No config.toml"

# Logs recientes
echo "=== RECENT LOGS ===" >> diagnosis.txt
tail -n 50 mechbot.log >> diagnosis.txt 2>/dev/null || echo "No logs"

# Hardware
echo "=== HARDWARE ===" >> diagnosis.txt
lsusb >> diagnosis.txt
i2cdetect -y 1 >> diagnosis.txt 2>/dev/null || echo "No I2C"

echo "✅ Diagnóstico guardado en diagnosis.txt"
```

---

**¿No encuentras solución aquí?** 
Consulta la documentación completa o contacta al equipo de soporte.
