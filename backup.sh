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
