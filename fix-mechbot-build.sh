#!/bin/bash

# ============================================================================
# 🔧 MechBot-3x Build Fixer
# Soluciona problemas de compilación de Rust
# ============================================================================

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

print_success() { echo -e "${GREEN}✓ $1${NC}"; }
print_error() { echo -e "${RED}✗ $1${NC}"; }
print_warning() { echo -e "${YELLOW}⚠ $1${NC}"; }
print_info() { echo -e "${CYAN}ℹ $1${NC}"; }

echo -e "${CYAN}"
echo "╔════════════════════════════════════════════════╗"
echo "║   🔧 MechBot-3x Build Environment Fixer       ║"
echo "╚════════════════════════════════════════════════╝"
echo -e "${NC}"

# 1. Deshabilitar RUSTUP_TOOLCHAIN en .bashrc
print_info "1️⃣ Deshabilitando RUSTUP_TOOLCHAIN en .bashrc..."
if grep -q "^export RUSTUP_TOOLCHAIN=" ~/.bashrc; then
    sed -i 's/^export RUSTUP_TOOLCHAIN=/#export RUSTUP_TOOLCHAIN=/' ~/.bashrc
    print_success "RUSTUP_TOOLCHAIN comentado en .bashrc"
else
    print_success "RUSTUP_TOOLCHAIN ya está deshabilitado"
fi

# 2. Eliminar override actual
print_info "2️⃣ Eliminando overrides de toolchain..."
unset RUSTUP_TOOLCHAIN
rustup override unset 2>/dev/null || true
print_success "Overrides eliminados"

# 3. Configurar stable como default
print_info "3️⃣ Configurando Rust stable..."
rustup default stable
RUST_VERSION=$(rustc --version)
print_success "Rust version: $RUST_VERSION"

# 4. Eliminar targets de Android que causan problemas
print_info "4️⃣ Limpiando targets de cross-compilation..."
if rustup target list --installed | grep -q "aarch64-linux-android"; then
    print_warning "Encontrado target aarch64-linux-android, eliminando..."
    rustup target remove aarch64-linux-android
    print_success "Target Android eliminado"
else
    print_success "No hay targets problemáticos"
fi

# 5. Verificar e instalar dependencias de Python
print_info "5️⃣ Verificando dependencias de Python..."
if ! dpkg -l | grep -q python3-dev; then
    print_warning "python3-dev no instalado, instalando..."
    sudo apt-get update -qq
    sudo apt-get install -y python3-dev pkg-config
    print_success "Dependencias de Python instaladas"
else
    print_success "python3-dev ya está instalado"
fi

# 6. Verificar archivo .cargo/config
print_info "6️⃣ Verificando configuración de Cargo..."
if [ -f ".cargo/config.toml" ] || [ -f ".cargo/config" ]; then
    CONFIG_FILE=".cargo/config.toml"
    [ ! -f "$CONFIG_FILE" ] && CONFIG_FILE=".cargo/config"
    
    if grep -q "aarch64-linux-android" "$CONFIG_FILE"; then
        print_warning "Encontrada configuración de Android en $CONFIG_FILE"
        print_info "Creando backup: ${CONFIG_FILE}.backup"
        cp "$CONFIG_FILE" "${CONFIG_FILE}.backup"
        
        # Comentar líneas de Android
        sed -i '/aarch64-linux-android/s/^/#/' "$CONFIG_FILE"
        print_success "Configuración de Android deshabilitada"
    else
        print_success "Configuración de Cargo correcta"
    fi
else
    print_success "No hay archivo de configuración de Cargo"
fi

# 7. Limpiar completamente
print_info "7️⃣ Limpiando proyecto..."
cargo clean
rm -rf Cargo.lock target/
print_success "Proyecto limpiado"

# 8. Actualizar dependencias
print_info "8️⃣ Actualizando dependencias..."
cargo update
print_success "Dependencias actualizadas"

# 9. Configurar variables de entorno para PyO3
print_info "9️⃣ Configurando entorno de compilación..."
export PYO3_PYTHON=$(which python3)
export PYTHON_SYS_EXECUTABLE=$(which python3)
print_success "Variables de entorno configuradas"

# 10. Compilar para el target nativo
print_info "🔟 Compilando proyecto..."
echo ""
print_warning "Esto puede tardar varios minutos..."
echo ""

if cargo build --release --target x86_64-unknown-linux-gnu; then
    echo ""
    print_success "¡Compilación exitosa! 🎉"
    echo ""
    print_info "Binario generado en:"
    echo "  target/x86_64-unknown-linux-gnu/release/"
    echo ""
    
    # Crear symlink para facilitar acceso
    if [ -d "target/x86_64-unknown-linux-gnu/release" ]; then
        ln -sf target/x86_64-unknown-linux-gnu/release target/release
        print_success "Symlink creado: target/release -> target/x86_64-unknown-linux-gnu/release"
    fi
else
    echo ""
    print_error "Falló la compilación"
    print_info "Intentando compilación sin optimización para ver errores detallados..."
    cargo build --target x86_64-unknown-linux-gnu
fi

echo ""
print_info "═══════════════════════════════════════════════"
print_info "Configuración completa. Reinicia tu terminal o ejecuta:"
echo "  source ~/.bashrc"
print_info "═══════════════════════════════════════════════"
