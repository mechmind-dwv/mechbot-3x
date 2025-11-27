#!/bin/bash
echo "🚀 Configurando MechBot-3x..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env
git clone https://github.com/mechmind-dwv/mechbot-3x.git
cd mechbot-3x
cargo build
echo "✅ Configuración completada!"
