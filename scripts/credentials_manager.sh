#!/bin/bash

# Manager seguro de credenciales
SECRETS_FILE=".secrets.toml"
BACKUP_DIR="secrets_backup"

echo "🔐 Gestor de Credenciales - MechBot-3x"

# Función para encriptar/desencriptar (simple base64 para demo)
encrypt() {
    echo "$1" | base64
}

decrypt() {
    echo "$1" | base64 -d 2>/dev/null
}

# Verificar si secrets file existe
if [ ! -f "$SECRETS_FILE" ]; then
    echo "📝 Creando nuevo archivo de secrets..."
    cat > "$SECRETS_FILE" << SECRETS
# Archivo de credenciales - NO COMMITAR ESTE ARCHIVO
# Agregar a .gitignore

[api]
# API Keys
main_key = "$(openssl rand -base64 32 | base64)"

[database]
# Credenciales de BD (si se usan)
# username = ""
# password = ""

[external_services]
# Claves para servicios externos
# openweather_api_key = ""
# google_maps_api_key = ""
SECRETS
    echo "✅ $SECRETS_FILE creado"
    echo "⚠️  Agrega '$SECRETS_FILE' a .gitignore"
fi

# Menú interactivo
while true; do
    echo ""
    echo "🔐 OPCIONES:"
    echo "1) Ver credenciales"
    echo "2) Agregar nueva credencial" 
    echo "3) Backup de secrets"
    echo "4) Restaurar backup"
    echo "5) Salir"
    read -p "Selecciona opción [1-5]: " choice

    case $choice in
        1)
            echo ""
            echo "📋 Credenciales actuales:"
            cat "$SECRETS_FILE" | grep -v "^#" | grep -v "^$"
            ;;
        2)
            read -p "🔑 Nombre de la credencial (ej: github_token): " key_name
            read -sp "📝 Valor: " key_value
            echo
            # Agregar al archivo
            echo "${key_name} = \"$(encrypt "$key_value")\"" >> "$SECRETS_FILE"
            echo "✅ Credencial agregada"
            ;;
        3)
            mkdir -p "$BACKUP_DIR"
            backup_file="$BACKUP_DIR/secrets_$(date +%Y%m%d_%H%M%S).toml.gpg"
            # Encriptar backup con GPG si está disponible
            if command -v gpg &> /dev/null; then
                gpg --symmetric --cipher-algo AES256 -o "$backup_file" "$SECRETS_FILE"
                echo "✅ Backup encriptado: $backup_file"
            else
                cp "$SECRETS_FILE" "$BACKUP_DIR/secrets_$(date +%Y%m%d_%H%M%S).toml"
                echo "✅ Backup creado (sin encriptar)"
            fi
            ;;
        4)
            echo "📂 Backups disponibles:"
            ls -la "$BACKUP_DIR/"*.toml* 2>/dev/null || echo "No hay backups"
            read -p "🔍 Nombre del backup a restaurar: " backup_file
            if [ -f "$BACKUP_DIR/$backup_file" ]; then
                cp "$BACKUP_DIR/$backup_file" "$SECRETS_FILE"
                echo "✅ Backup restaurado"
            else
                echo "❌ Backup no encontrado"
            fi
            ;;
        5)
            echo "👋 Saliendo..."
            exit 0
            ;;
        *)
            echo "❌ Opción inválida"
            ;;
    esac
done
