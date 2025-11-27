#!/bin/bash
# Health check script for MechBot-3x

set -e

echo "🏥 MechBot-3x Health Check"
echo "=========================="

# Check if service is running
if curl -f http://localhost:8080/api/v1/status > /dev/null 2>&1; then
    echo "✅ API Server: HEALTHY"
else
    echo "❌ API Server: UNHEALTHY"
    exit 1
fi

# Check WebSocket
if nc -z localhost 8081; then
    echo "✅ WebSocket Server: HEALTHY"
else
    echo "❌ WebSocket Server: UNHEALTHY"
    exit 1
fi

# Check disk space
DISK_USAGE=$(df / | awk 'NR==2 {print $5}' | sed 's/%//')
if [ $DISK_USAGE -lt 90 ]; then
    echo "✅ Disk Space: HEALTHY ($DISK_USAGE% used)"
else
    echo "⚠️ Disk Space: WARNING ($DISK_USAGE% used)"
fi

# Check memory
MEM_USAGE=$(free | awk 'NR==2{printf "%.2f", $3*100/$2}')
echo "📊 Memory Usage: $MEM_USAGE%"

echo "🎉 All health checks passed!"
