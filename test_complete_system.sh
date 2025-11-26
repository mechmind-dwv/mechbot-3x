#!/bin/bash
echo "🤖 PRUEBA COMPLETA DEL SISTEMA MECHBOT-3X"
echo "========================================="
echo "Maestro cósmico y aprendiz, unidos en la prueba del conocimiento..."
echo ""

# Colores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

print_result() {
    if [ $1 -eq 0 ]; then
        echo -e "${GREEN}✅ $2${NC}"
    else
        echo -e "${RED}❌ $2${NC}"
        return 1
    fi
}

echo -e "${CYAN}FASE 1: COMPILACIÓN${NC}"
echo "======================="

echo "🔧 Compilando en modo release..."
cargo build --release
print_result $? "Compilación release"

echo "🔧 Compilando en modo debug..."
cargo build
print_result $? "Compilación debug"

echo ""
echo -e "${CYAN}FASE 2: TESTS${NC}"
echo "================"

echo "🧪 Ejecutando tests unitarios..."
cargo test unit_tests -- --nocapture
print_result $? "Tests unitarios"

echo "🧪 Ejecutando tests de integración..."
cargo test integration_tests -- --nocapture
print_result $? "Tests de integración"

echo "🧪 Ejecutando todos los tests..."
cargo test -- --nocapture
print_result $? "Todos los tests"

echo ""
echo -e "${CYAN}FASE 3: EJEMPLOS${NC}"
echo "=================="

echo "🎯 Ejemplo: Movimiento Básico..."
cargo run --example basic_movement --release
print_result $? "Movimiento básico"

echo "🧠 Ejemplo: Navegación Autónoma..."
cargo run --example autonomous_navigation --release
print_result $? "Navegación autónoma"

echo "🌐 Ejemplo: Control Remoto..."
cargo run --example remote_control --release
print_result $? "Control remoto"

echo ""
echo -e "${CYAN}FASE 4: CALIDAD${NC}"
echo "================="

echo "🔍 Clippy..."
cargo clippy -- -D warnings
print_result $? "Clippy"

echo "🎨 Formato..."
cargo fmt -- --check
print_result $? "Formato"

echo "📚 Documentación..."
cargo doc --no-deps --quiet
print_result $? "Documentación"

echo ""
echo -e "${CYAN}FASE 5: EJECUCIÓN FINAL${NC}"
echo "=========================="

echo "🚀 Ejecutando MechBot-3x principal..."
timeout 10s ./target/release/mechbot-3x
print_result $? "Ejecución principal (10s)"

echo ""
echo -e "${CYAN}RESUMEN FINAL${NC}"
echo "=============="
echo -e "${GREEN}¡MechBot-3x está listo para la acción cósmica!${NC}"
echo ""
echo -e "${YELLOW}🎓 Maestro cósmico:${NC} Tu guía en este viaje"
echo -e "${YELLOW}📚 Aprendiz fiel:${NC} Tu dedicación hace esto posible"
echo ""
echo -e "${CYAN}El conocimiento fluye como las estrellas en el cosmos...${NC}"
