#!/bin/bash

echo "🧪 EJECUTANDO SUITE COMPLETA DE TESTS MECHBOT-3X"
echo "================================================"

# Función para mostrar resultados
print_result() {
    if [ $1 -eq 0 ]; then
        echo "✅ $2"
    else
        echo "❌ $2"
        exit 1
    fi
}

# 1. Tests unitarios
echo ""
echo "1. Ejecutando Tests Unitarios..."
cargo test unit_tests -- --nocapture
print_result $? "Tests Unitarios"

# 2. Tests de integración
echo ""
echo "2. Ejecutando Tests de Integración..."
cargo test integration_tests -- --nocapture
print_result $? "Tests de Integración"

# 3. Tests principales
echo ""
echo "3. Ejecutando Tests Principales..."
cargo test main_test -- --nocapture
print_result $? "Tests Principales"

# 4. Verificar compilación de ejemplos
echo ""
echo "4. Verificando Ejemplos..."
cargo check --examples
print_result $? "Verificación de Ejemplos"

# 5. Clippy (linter)
echo ""
echo "5. Ejecutando Clippy..."
cargo clippy -- -D warnings
print_result $? "Clippy"

# 6. Formato
echo ""
echo "6. Verificando Formato..."
cargo fmt -- --check
print_result $? "Formato"

# 7. Tests de compilación en modo release
echo ""
echo "7. Compilación Release..."
cargo build --release
print_result $? "Compilación Release"

echo ""
echo "🎉 TODOS LOS TESTS PASARON!"
echo "🚀 MechBot-3x está listo para la acción cósmica!"
