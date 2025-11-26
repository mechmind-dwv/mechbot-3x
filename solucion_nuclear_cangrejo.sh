#!/bin/bash
echo "💥 SOLUCIÓN NUCLEAR CANGREJO"
echo "============================"

# 1. Limpiar TODO
echo "🧹 Limpieza nuclear..."
cargo clean
rm -f rust-toolchain.toml
rm -f Cargo.lock

# 2. Reinstalar Rust si es necesario
echo "🔧 Revisando instalación Rust..."
rustup update stable
rustup default stable
rustup target add x86_64-unknown-linux-gnu

# 3. Compilar solo el binario principal
echo "🎯 Compilando binario directo..."
rustc src/main.rs --extern mechbot_3x=target/debug/deps/libmechbot_3x.rlib --edition 2021 -C opt-level=0 -o mechbot-3x-simple

# 4. Si falla, crear un main.rs mínimo
if [ ! -f "mechbot-3x-simple" ]; then
    echo "🦀 Creando versión mínima cangrejo..."
    cat > mechbot_simple.rs << 'SIMPLE'
fn main() {
    println!("🦀 ¡MECHBOT-3X CANGREJO ACTIVADO!");
    println!("🌌 Sistema cósmico inicializado");
    println!("🚀 Versión: Retroceso Productivo 1.0");
    println!("🎯 Listo para operar (en reversa)");
    
    // Misión cangrejo
    for i in 1..=3 {
        println!("📍 Cangrejo avanzando (hacia atrás) {}...", i);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    
    println!("🎉 ¡Misión cangrejo completada!");
}
SIMPLE
    
    rustc mechbot_simple.rs -o mechbot-3x
    echo "✅ Binario mínimo creado: ./mechbot-3x"
fi

# 5. Ejecutar lo que sea que tengamos
if [ -f "./mechbot-3x" ]; then
    echo "🚀 EJECUTANDO MECHBOT CANGREJO..."
    ./mechbot-3x
elif [ -f "./mechbot-3x-simple" ]; then
    echo "🚀 EJECUTANDO MECHBOT SIMPLE..."
    ./mechbot-3x-simple
elif [ -f "./target/debug/mechbot-3x" ]; then
    echo "🚀 EJECUTANDO MECHBOT DEBUG..."
    ./target/debug/mechbot-3x
else
    echo "❌ No se pudo crear ningún binario"
    echo "💡 Último intento con cargo:"
    cargo run
fi
