#!/bin/bash
echo "🔧 REPARANDO MECHBOT-3X - SOLUCIÓN CÓSMICA"
echo "=========================================="

# 1. Arreglar Cargo.toml
echo "📦 Reparando Cargo.toml..."
cat > Cargo.toml << 'CARGO_EOF'
[package]
name = "mechbot-3x"
version = "0.1.0"
edition = "2021"

[dependencies]
anyhow = "1.0"
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
toml = "0.8"
log = "0.4"
env_logger = "0.10"
rand = "0.8"

[[bin]]
name = "mechbot-3x"
path = "src/main.rs"

[lib]
name = "mechbot_3x"
path = "src/lib.rs"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1

[dev-dependencies]
tokio = { version = "1.0", features = ["full", "test-util"] }
criterion = "0.5"

[[test]]
name = "main_test"
path = "tests/main_test.rs"

[[test]]
name = "unit_tests"
path = "tests/unit/mod.rs"

[[test]]
name = "integration_tests"
path = "tests/integration/mod.rs"

[[example]]
name = "basic_movement"
path = "examples/basic_movement.rs"

[[example]]
name = "autonomous_navigation"
path = "examples/autonomous_navigation.rs"

[[example]]
name = "remote_control"
path = "examples/remote_control.rs"

[profile.test]
opt-level = 0
debug = true

[profile.bench]
opt-level = 3
debug = false
CARGO_EOF

echo "✅ Cargo.toml reparado"

# 2. Arreglar campos privados en NavigationController
echo "🛠️ Agregando getters a NavigationController..."
cat >> src/navigation/mod.rs << 'NAV_EOF'

// Métodos de acceso público para campos privados
impl NavigationController {
    pub fn get_current_position(&self) -> (f64, f64) {
        self.current_position
    }
    
    pub fn get_target_position(&self) -> (f64, f64) {
        self.target_position
    }
    
    pub fn get_current_path(&self) -> &Vec<(f64, f64)> {
        &self.path
    }
    
    pub fn set_current_position(&mut self, x: f64, y: f64) {
        self.current_position = (x, y);
    }
}
NAV_EOF

# 3. Arreglar robot.rs
echo "🤖 Reparando accesos en robot.rs..."
if grep -q "self.navigation.current_position" src/robot.rs; then
    sed -i 's/self\.navigation\.current_position/self\.navigation\.get_current_position()/g' src/robot.rs
    echo "✅ Campos privados reparados en robot.rs"
else
    echo "ℹ️  No se encontraron accesos directos a campos privados"
fi

# 4. Verificar compilación
echo "🔍 Verificando compilación..."
cargo clean
if cargo check; then
    echo "🎉 ¡COMPILACIÓN EXITOSA!"
else
    echo "❌ Aún hay errores. Mostrando detalles:"
    cargo check
    exit 1
fi

echo ""
echo "🚀 MECHBOT-3X REPARADO Y LISTO PARA ACCIÓN CÓSMICA!"
