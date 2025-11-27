FROM rust:1.83-slim-bullseye as builder

# Instalar dependencias del sistema
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libopencv-dev \
    clang \
    libclang-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copiar archivos de dependencias primero (para cache de Docker)
COPY Cargo.toml Cargo.lock ./
COPY src/ ./src/

# Build en modo release
RUN cargo build --release

# Runtime image
FROM debian:bullseye-slim

# Instalar runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl1.1 \
    libopencv-core4.5 \
    libopencv-imgproc4.5 \
    libopencv-imgcodecs4.5 \
    && rm -rf /var/lib/apt/lists/*

# Crear usuario no-root
RUN useradd -m -u 1000 mechbot

WORKDIR /app

# Copiar binario desde builder
COPY --from=builder /app/target/release/mechbot-3x /app/
COPY config.toml /app/
COPY scripts/ /app/scripts/

# Crear directorios necesarios
RUN mkdir -p /app/logs /app/data

# Permisos
RUN chown -R mechbot:mechbot /app
RUN chmod +x /app/mechbot-3x /app/scripts/*.sh

USER mechbot

# Exponer puertos
EXPOSE 8080 8081

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/api/v1/status || exit 1

# Comando de inicio
CMD ["./mechbot-3x"]
