#!/bin/bash

echo "🧪 Ejecutando tests de MechBot-3x..."

# Tests unitarios
echo "📦 Ejecutando tests unitarios..."
cargo test --lib -- --test-threads=1

# Tests de integración
echo "🔗 Ejecutando tests de integración..."
cargo test --test integration_tests -- --test-threads=1

# Tests doc
echo "📚 Ejecutando tests de documentación..."
cargo test --doc

# Clippy para calidad de código
echo "🔍 Ejecutando Clippy..."
cargo clippy -- -D warnings

# Formato de código
echo "🎨 Verificando formato..."
cargo fmt -- --check

echo "✅ Todos los tests completados!"
