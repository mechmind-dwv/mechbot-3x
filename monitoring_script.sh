#!/bin/bash
# monitoring_script.sh

echo "🔍 Monitoreando MechBot-3x..."

while true; do
    STATUS=$(curl -s http://localhost:8080/api/v1/status)
    BATTERY=$(echo $STATUS | grep -o '"battery_level":[0-9]*' | cut -d: -f2)
    
    echo "Batería: ${BATTERY}% - $(date)"
    
    if [ "$BATTERY" -lt 20 ]; then
        echo "⚠️  Batería baja! Conectar a carga."
    fi
    
    sleep 30
done
