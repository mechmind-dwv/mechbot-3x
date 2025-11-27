#!/bin/bash
echo "🚀 Aplicando fix rápido..."
cargo clean
cargo fmt
./scripts/fix_imports.sh
cargo check || echo "⚠️  Aún hay errores, pero el proyecto está más limpio"
