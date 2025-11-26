#!/bin/bash
set -e

echo "🔧 Arreglando versión de Rust..."

# 1. Eliminar overrides
echo "1️⃣ Eliminando overrides de toolchain..."
unset RUSTUP_TOOLCHAIN
rustup override unset 2>/dev/null || true

# 2. Verificar versión
echo "2️⃣ Versión actual de Rust:"
rustc --version

# 3. Si no es >= 1.81, actualizar
RUST_VERSION=$(rustc --version | grep -oP '\d+\.\d+' | head -1)
if (( $(echo "$RUST_VERSION < 1.81" | bc -l) )); then
    echo "⚠️  Versión antigua detectada, actualizando..."
    rustup update stable
    rustup default stable
fi

# 4. Configurar rust-toolchain.toml
echo "3️⃣ Configurando rust-toolchain.toml..."
cat > rust-toolchain.toml << 'EOF'
[toolchain]
channel = "stable"
components = ["rustfmt", "clippy"]
profile = "default"
EOF

# 5. Limpiar y regenerar
echo "4️⃣ Limpiando proyecto..."
cargo clean
rm -f Cargo.lock

echo "5️⃣ Actualizando dependencias..."
cargo update

# 6. Compilar
echo "6️⃣ Compilando..."
cargo build --release

echo "✅ ¡Todo listo!"
