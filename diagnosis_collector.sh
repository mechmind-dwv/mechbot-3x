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
