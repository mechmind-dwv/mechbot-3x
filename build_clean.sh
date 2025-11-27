#!/bin/bash
# Script de compilación limpia - ignora todas las variables de entorno problemáticas

# Desactivar variables problemáticas
unset RUSTUP_TOOLCHAIN
unset CARGO_BUILD_TARGET

# Forzar toolchain estable
export RUSTUP_TOOLCHAIN=stable

# Compilar
cargo "$@"
