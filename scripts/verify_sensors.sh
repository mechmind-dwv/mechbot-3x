#!/bin/bash
# Script de verificación de sensores MechBot-3X

echo "🔍 Verificación de Sensores MechBot-3X"
echo "======================================"

# Verificar LIDAR
echo ""
echo "📡 Verificando LIDAR..."
if [ -c "/dev/ttyUSB0" ]; then
    echo "✅ LIDAR detectado: /dev/ttyUSB0"
    # Verificar permisos
    if [ -r "/dev/ttyUSB0" ]; then
        echo "✅ Permisos de lectura OK"
    else
        echo "❌ Sin permisos de lectura en /dev/ttyUSB0"
        echo "💡 Ejecutar: sudo usermod -a -G dialout $USER"
    fi
else
    echo "❌ LIDAR no detectado en /dev/ttyUSB0"
    echo "💡 Verificar conexión USB y drivers"
fi

# Verificar IMU
echo ""
echo "🧭 Verificando IMU..."
if command -v i2cdetect &> /dev/null; then
    echo "✅ i2cdetect disponible"
    if i2cdetect -y 1 | grep -q "68"; then
        echo "✅ IMU MPU6050 detectado en dirección 0x68"
    else
        echo "❌ IMU no detectado en bus I2C"
        echo "💡 Verificar conexiones: SDA=GPIO2, SCL=GPIO3, 3.3V, GND"
    fi
else
    echo "⚠️ i2cdetect no disponible, instalar: sudo apt install i2c-tools"
fi

# Verificar cámara
echo ""
echo "📷 Verificando cámara..."
if command -v v4l2-ctl &> /dev/null; then
    if v4l2-ctl --list-devices | grep -q "video0"; then
        echo "✅ Cámara detectada: /dev/video0"
        # Verificar resolución
        RESOLUTION=$(v4l2-ctl --list-formats-ext | grep -o "[0-9]*x[0-9]*" | head -1)
        if [ ! -z "$RESOLUTION" ]; then
            echo "✅ Resolución soportada: $RESOLUTION"
        fi
    else
        echo "❌ Cámara no detectada"
    fi
else
    echo "⚠️ v4l2-ctl no disponible, instalar: sudo apt install v4l-utils"
fi

# Verificar dependencias Rust
echo ""
echo "🦀 Verificando dependencias Rust..."
if command -v cargo &> /dev/null; then
    echo "✅ Cargo disponible"
else
    echo "❌ Cargo no disponible"
    echo "💡 Instalar Rust: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
fi

echo ""
echo "🎉 Verificación completada"
echo "💡 Para probar los sensores ejecutar: cargo run --example sensor_integration"
