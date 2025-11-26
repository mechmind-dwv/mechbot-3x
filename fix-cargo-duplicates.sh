#!/bin/bash
echo "🔧 Reparando Cargo.toml con claves duplicadas..."

# Backup del Cargo.toml actual
cp Cargo.toml Cargo.toml.backup

# Crear Cargo.toml limpio (Opción A: solo binario)
cat > Cargo.toml << 'CARGO_EOF'
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
CARGO_EOF

echo "✅ Cargo.toml limpiado"

# Preguntar si mantener lib.rs
if [ -f "src/lib.rs" ]; then
    echo "📚 Encontrado src/lib.rs"
    read -p "¿Quieres mantenerlo como librería? (s/n): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Ss]$ ]]; then
        # Opción con librería
        cat > Cargo.toml << 'CARGO_LIB_EOF'
[package]
name = "mechbot-3x"
version = "0.1.0"
edition = "2021"

[dependencies]
anyhow = "1.0"
toml = "0.8"
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }

[lib]
name = "mechbot_3x"
path = "src/lib.rs"

[[bin]]
name = "mechbot-3x"
path = "src/main.rs"
CARGO_LIB_EOF
        
        # Crear lib.rs básico si está vacío o corrupto
        if [ ! -s "src/lib.rs" ]; then
            cat > src/lib.rs << 'LIB_EOF'
pub mod config;
pub mod robot;

pub use config::Config;
pub use robot::Robot;
LIB_EOF
        fi
        echo "✅ Configurado como librería + binario"
    else
        rm src/lib.rs
        echo "✅ Eliminado lib.rs (solo binario)"
    fi
fi

# Compilar
echo "🔄 Compilando..."
cargo clean
cargo build

if [ $? -eq 0 ]; then
    echo "🎉 ¡Compilación exitosa!"
    echo "🤖 Ejecuta: ./target/debug/mechbot-3x"
else
    echo "❌ Error en compilación"
    cargo check
fi
