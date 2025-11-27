#!/bin/bash
# Script de construcción robusto para MechBot-3x

set -e

echo "🔧 Building MechBot-3x..."

# Colores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Función para imprimir con color
print_status() {
    echo -e "${GREEN}✅${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠️${NC} $1"
}

print_error() {
    echo -e "${RED}❌${NC} $1"
}

# Verificar Rust
if ! command -v rustc &> /dev/null; then
    print_error "Rust no encontrado. Instala Rust primero:"
    echo "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

# Verificar toolchain
RUST_VERSION=$(rustc --version | cut -d' ' -f2)
print_status "Rust version: $RUST_VERSION"

# Clean previo (opcional)
if [ "$1" = "--clean" ]; then
    print_status "Cleaning previous build..."
    cargo clean
fi

# Check formato
print_status "Checking code formatting..."
if ! cargo fmt -- --check; then
    print_warning "Code formatting issues found. Run 'cargo fmt' to fix."
fi

# Clippy check
print_status "Running clippy checks..."
if ! cargo clippy -- -D warnings; then
    print_error "Clippy checks failed"
    exit 1
fi

# Build en modo debug primero (más rápido para desarrollo)
print_status "Building in debug mode..."
if ! cargo build; then
    print_error "Debug build failed"
    exit 1
fi

# Build en modo release
print_status "Building in release mode..."
if ! cargo build --release; then
    print_error "Release build failed"
    exit 1
fi

# Run tests
print_status "Running tests..."
if ! cargo test; then
    print_error "Tests failed"
    exit 1
fi

# Verificar que el binario se creó
if [ -f "./target/release/mechbot-3x" ]; then
    print_status "Build successful! Binary created: ./target/release/mechbot-3x"
    
    # Mostrar información del binario
    BINARY_SIZE=$(du -h "./target/release/mechbot-3x" | cut -f1)
    print_status "Binary size: $BINARY_SIZE"
else
    print_error "Binary not found at ./target/release/mechbot-3x"
    exit 1
fi

# Verificar dependencias opcionales
print_status "Checking optional dependencies..."

# Verificar Docker
if command -v docker &> /dev/null; then
    DOCKER_VERSION=$(docker --version | cut -d' ' -f3 | tr -d ',')
    print_status "Docker: $DOCKER_VERSION"
else
    print_warning "Docker not found - container deployment will not work"
fi

# Verificar Kubernetes
if command -v kubectl &> /dev/null; then
    KUBE_VERSION=$(kubectl version --client -o json | jq -r '.clientVersion.gitVersion')
    print_status "Kubernetes: $KUBE_VERSION"
else
    print_warning "kubectl not found - Kubernetes deployment will not work"
fi

print_status "🎉 All builds completed successfully!"
echo ""
echo "🚀 Next steps:"
echo "   ./target/release/mechbot-3x          # Run the application"
echo "   ./scripts/deploy.sh development      # Deploy with Docker"
echo "   cargo run --example sensor_integration  # Run examples"
