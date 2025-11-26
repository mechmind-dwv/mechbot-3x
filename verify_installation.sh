#!/bin/bash
echo "🔍 Verificando instalación de MechBot-3x..."

# Verificar Rust
if command -v rustc &> /dev/null; then
    echo "✅ Rust instalado: $(rustc --version)"
else
    echo "❌ Rust no instalado"
fi

# Verificar compilación
if [ -f "target/release/mechbot-3x" ]; then
    echo "✅ Binario compilado correctamente"
else
    echo "❌ Binario no encontrado, compilar con: cargo build --release"
fi

# Verificar configuración
if [ -f "config.toml" ]; then
    echo "✅ Archivo de configuración encontrado"
else
    echo "⚠️  Crear config.toml desde config_minimal.toml"
fi

# Verificar APIs
if curl -s http://localhost:8080/api/v1/status > /dev/null; then
    echo "✅ API REST funcionando"
else
    echo "❌ API REST no responde"
fi

echo "🎉 Verificación completada!"
