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
