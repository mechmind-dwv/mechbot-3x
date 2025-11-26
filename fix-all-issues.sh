#!/bin/bash
echo "🔧 Reparando MechBot-3x..."

# Paso 1: Crear Cargo.toml
cat > Cargo.toml << 'CARGO'
[package]
name = "mechbot-3x"
version = "0.1.0"
edition = "2021"

[dependencies]
anyhow = "1.0"
toml = "0.8"
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }

[[bin]]
name = "mechbot-3x"
path = "src/main.rs"
CARGO

# Paso 2: Corregir rust-toolchain
cat > rust-toolchain.toml << 'TOOLCHAIN'
[toolchain]
channel = "stable"
components = ["rustfmt", "clippy", "rust-analyzer"]
targets = ["x86_64-unknown-linux-gnu"]
profile = "default"
TOOLCHAIN

# Paso 3: Actualizar Rust
rustup update stable
rustup target add x86_64-unknown-linux-gnu
rustup component add rustfmt clippy rust-analyzer

# Paso 4: Corregir imports en config.rs
if [ -f src/config.rs ]; then
    sed -i '1s/^/use anyhow::{bail, Context, Result};\n/' src/config.rs
    sed -i 's/tmp_enabled/tpm_enabled/g' src/config.rs
    sed -i 's/PowerConfig/ModelConfig/g' src/config.rs 2>/dev/null || true
fi

# Paso 5: Compilar
echo "🔄 Compilando..."
cargo clean
cargo build

echo "✅ ¡Reparación completada!"
