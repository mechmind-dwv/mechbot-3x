#!/bin/bash

echo "🎯 REPARANDO PROBLEMA ANDROID DEFINITIVAMENTE"
echo "============================================="

# Colores
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# 1. Detectar configuraciones problemáticas
echo -e "${BLUE}1. Buscando configuraciones Android...${NC}"
PROBLEM_FILES=$(find . -name "*.toml" -o -name "*.json" -o -name "*.config" 2>/dev/null | xargs grep -l "android\|aarch64" 2>/dev/null || true)

if [ ! -z "$PROBLEM_FILES" ]; then
    echo -e "${YELLOW}⚠️  Archivos problemáticos encontrados:${NC}"
    echo "$PROBLEM_FILES"
    echo ""
    read -p "¿Eliminar estos archivos? [y/N]: " confirm
    if [[ $confirm =~ ^[Yy]$ ]]; then
        echo "$PROBLEM_FILES" | xargs rm -f
        echo -e "${GREEN}✅ Archivos eliminados${NC}"
    fi
else
    echo -e "${GREEN}✅ No se encontraron archivos problemáticos${NC}"
fi

# 2. Limpiar variables de entorno
echo -e "${BLUE}2. Limpiando variables de entorno...${NC}"
unset CARGO_BUILD_TARGET 2>/dev/null || true
unset RUSTUP_TOOLCHAIN 2>/dev/null || true
unset RUST_TARGET 2>/dev/null || true
echo -e "${GREEN}✅ Variables limpiadas${NC}"

# 3. Resetear Rust
echo -e "${BLUE}3. Reseteando configuración Rust...${NC}"
rustup override unset 2>/dev/null || true
rustup default stable
echo -e "${GREEN}✅ Rust resetado${NC}"

# 4. Configurar target correcto
echo -e "${BLUE}4. Configurando target Linux...${NC}"
rustup target add x86_64-unknown-linux-gnu
echo -e "${GREEN}✅ Target Linux configurado${NC}"

# 5. Verificar estado
echo -e "${BLUE}5. Verificando estado...${NC}"
echo "Rust version: $(rustc --version)"
echo "Default host: $(rustc -vV | grep host | cut -d' ' -f2)"
echo "Active toolchain: $(rustup show active-toolchain)"

# 6. Compilar
echo -e "${BLUE}6. Compilando proyecto...${NC}"
cargo clean
cargo build --target x86_64-unknown-linux-gnu --release

# 7. Verificar resultado
if [ -f "target/x86_64-unknown-linux-gnu/release/mechbot-3x" ]; then
    echo -e "${GREEN}🎉 ¡COMPILACIÓN EXITOSA!${NC}"
    echo -e "Binario: ${GREEN}target/x86_64-unknown-linux-gnu/release/mechbot-3x${NC}"
    
    # Crear symlink para fácil acceso
    ln -sf target/x86_64-unknown-linux-gnu/release/mechbot-3x mechbot-3x
    echo -e "Symlink: ${GREEN}./mechbot-3x${NC}"
    
    # Probar ejecución
    echo -e "${BLUE}Probando ejecución...${NC}"
    ./mechbot-3x --version || echo -e "${YELLOW}⚠️  Ejecución falló, pero el binario existe${NC}"
else
    echo -e "${RED}❌ Compilación falló${NC}"
    echo "Últimos errores:"
    cargo check --target x86_64-unknown-linux-gnu 2>&1 | tail -10
fi
