FROM ubuntu:20.04

# Instalar dependencias del sistema
RUN apt-get update && apt-get install -y \
    build-essential \
    cmake \
    pkg-config \
    libopencv-dev \
    libusb-1.0-0-dev \
    libssl-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Instalar Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Copiar código y compilar
WORKDIR /app
COPY . .
RUN cargo build --release

# Configurar runtime
USER 1000:1000
EXPOSE 8080 8081
CMD ["./target/release/mechbot-3x"]
