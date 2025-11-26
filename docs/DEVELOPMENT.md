# 💻 Guía de Desarrollo - MechBot-3x

Esta guía está dirigida a desarrolladores que quieran contribuir al proyecto o desarrollar extensiones.

## 🛠️ Configuración del Entorno

### Requisitos de Desarrollo
```bash
# Herramientas básicas
sudo apt install -y git curl wget build-essential cmake pkg-config

# Herramientas Rust
rustup component add rustfmt clippy rust-analyzer

# Dependencias del proyecto
sudo apt install -y \
    libopencv-dev \
    libusb-1.0-0-dev \
    libssl-dev \
    libasound2-dev \
    libavcodec-dev \
    libavformat-dev \
    libswscale-dev

# Herramientas de debugging
sudo apt install -y gdb valgrind perf
```

### Configuración de Git
```bash
git config --global user.name "Tu Nombre"
git config --global user.email "tu.email@ejemplo.com"
git config --global pull.rebase true

# Hooks de git (opcional pero recomendado)
cp scripts/git-hooks/* .git/hooks/
chmod +x .git/hooks/*
```

## 🏗️ Estructura del Proyecto

```
mechbot-3x/
├── src/
│   ├── main.rs              # Punto de entrada
│   ├── lib.rs              # Librería principal
│   ├── config.rs           # Manejo de configuración
│   ├── robot.rs            # Lógica principal del robot
│   ├── sensors/            # Módulo de sensores
│   │   ├── mod.rs
│   │   ├── lidar.rs
│   │   ├── imu.rs
│   │   └── camera.rs
│   ├── navigation/         # Módulo de navegación
│   │   ├── mod.rs
│   │   ├── pathfinding.rs
│   │   └── control.rs
│   ├── vision/            # Módulo de visión
│   │   ├── mod.rs
│   │   ├── detection.rs
│   │   └── tracking.rs
│   └── api/               # Módulo de API
│       ├── mod.rs
│       ├── rest.rs
│       └── websocket.rs
├── tests/                 # Tests de integración
├── examples/              # Ejemplos de uso
├── benches/               # Benchmarks
├── scripts/               # Scripts de desarrollo
└── docs/                  # Documentación
```

## 🔧 Flujo de Desarrollo

### 1. Fork y Clone
```bash
# Hacer fork en GitHub, luego:
git clone https://github.com/tu-usuario/mechbot-3x.git
cd mechbot-3x

# Agregar upstream
git remote add upstream https://github.com/mechmind-dwv/mechbot-3x.git
```

### 2. Crear Rama de Feature
```bash
git checkout -b feature/nueva-funcionalidad
```

### 3. Desarrollar con Tests
```bash
# Ejecutar tests continuamente
cargo watch -x test

# Ejecutar tests específicos
cargo test test_navigation
cargo test -- --nocapture  # Ver output

# Benchmarks
cargo bench
```

### 4. Verificar Calidad de Código
```bash
# Formatear código
cargo fmt

# Linter
cargo clippy -- -D warnings

# Verificar compilación
cargo check

# Security audit
cargo audit
```

### 5. Commit y Push
```bash
git add .
git commit -m "feat: agregar nueva funcionalidad

- Descripción detallada de los cambios
- Breaking changes si los hay
- Referencias a issues"

git push origin feature/nueva-funcionalidad
```

## 🧪 Testing

### Tests Unitarios
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_navigation_planning() {
        let planner = AStarPlanner::new();
        let path = planner.plan_path(Point::new(0, 0), Point::new(10, 10));
        assert!(path.is_ok());
    }

    #[tokio::test]
    async fn test_sensor_reading() {
        let sensor = MockSensor::new();
        let reading = sensor.read().await;
        assert_eq!(reading.value, 42);
    }
}
```

### Tests de Integración
```rust
// tests/integration_test.rs
use mechbot_3x::{Robot, Config};

#[tokio::test]
async fn test_robot_initialization() {
    let config = Config::default();
    let robot = Robot::new(config).await;
    assert!(robot.is_ok());
}
```

### Mocks para Testing
```rust
// tests/mocks.rs
pub struct MockLidar {
    data: Vec<Point>,
}

impl MockLidar {
    pub fn new() -> Self {
        Self { data: vec![] }
    }
    
    pub fn with_sample_data() -> Self {
        Self { 
            data: vec![Point::new(1.0, 0.0), Point::new(2.0, 0.0)] 
        }
    }
}

#[async_trait]
impl Lidar for MockLidar {
    async fn scan(&mut self) -> Result<Vec<Point>> {
        Ok(self.data.clone())
    }
}
```

## 🔍 Debugging

### Debug con GDB
```bash
# Compilar con símbolos de debug
cargo build

# Ejecutar con GDB
gdb target/debug/mechbot-3x

# Comandos útiles de GDB
(gdb) break main
(gdb) run
(gdb) next
(gdb) print variable
(gdb) backtrace
```

### Debug con LLDB
```bash
cargo build
lldb target/debug/mechbot-3x
```

### Logging para Debug
```rust
use log::{debug, info, warn, error};

pub fn complex_algorithm(input: &str) -> Result<()> {
    debug!("Starting algorithm with input: {}", input);
    
    let result = process_input(input)?;
    info!("Algorithm completed successfully");
    
    Ok(result)
}
```

## 📊 Performance y Profiling

### Benchmarking
```rust
// benches/navigation_bench.rs
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_path_planning(c: &mut Criterion) {
    c.bench_function("a_star_100x100", |b| {
        b.iter(|| {
            let planner = AStarPlanner::new();
            planner.plan_path(Point::new(0, 0), Point::new(99, 99))
        })
    });
}

criterion_group!(benches, bench_path_planning);
criterion_main!(benches);
```

### Profiling con Flamegraph
```bash
cargo install flamegraph
cargo flamegraph --bin mechbot-3x --features profiling
```

### Memory Profiling
```bash
cargo install valgrind
valgrind --tool=massif ./target/debug/mechbot-3x
ms_print massif.out.*
```

## 📝 Convenciones de Código

### Estilo Rust
```rust
// Use snake_case para funciones y variables
pub fn calculate_distance(point_a: Point, point_b: Point) -> f64 {
    // ...
}

// Use PascalCase para tipos
pub struct RobotConfig {
    pub name: String,
    pub max_speed: f64,
}

// Documentación completa
/// Calcula la distancia entre dos puntos en el plano 2D.
///
/// # Arguments
/// * `point_a` - Primer punto
/// * `point_b` - Segundo punto
///
/// # Returns
/// Distancia euclidiana entre los puntos
///
/// # Examples
/// ```
/// let p1 = Point::new(0.0, 0.0);
/// let p2 = Point::new(3.0, 4.0);
/// assert_eq!(calculate_distance(p1, p2), 5.0);
/// ```
pub fn calculate_distance(point_a: Point, point_b: Point) -> f64 {
    point_a.distance_to(point_b)
}
```

### Gestión de Errores
```rust
use anyhow::{Context, Result};

pub async fn load_sensor_data() -> Result<SensorData> {
    let config = load_config()
        .context("Failed to load configuration")?;
    
    let data = read_sensor(&config)
        .await
        .context("Sensor reading failed")?;
        
    Ok(data)
}
```

## 🔌 Desarrollo de Módulos

### Crear un Nuevo Módulo
1. Crear directorio en `src/`
2. Agregar `mod.rs`
3. Exportar en `lib.rs`
4. Documentar la API pública

### Ejemplo: Módulo de Nuevo Sensor
```rust
// src/sensors/nuevo_sensor.rs
use anyhow::Result;

/// Sensor personalizado para MechBot-3x
pub struct NuevoSensor {
    // implementación
}

impl NuevoSensor {
    pub fn new(config: &SensorConfig) -> Result<Self> {
        // inicialización
    }
    
    pub async fn read(&mut self) -> Result<SensorReading> {
        // lectura de datos
    }
}

// src/sensors/mod.rs
pub mod nuevo_sensor;
pub use nuevo_sensor::NuevoSensor;

// src/lib.rs
pub mod sensors;
pub use sensors::NuevoSensor;
```

## 🚀 CI/CD y Quality Gates

### GitHub Actions
El proyecto usa GitHub Actions para:
- ✅ Tests automáticos en cada push
- ✅ Linting y formatting checks
- ✅ Security scanning
- ✅ Build verification

### Pre-commit Hooks
```bash
# scripts/git-hooks/pre-commit
#!/bin/bash
cargo fmt -- --check
cargo clippy -- -D warnings
cargo test --lib
```

## 📚 Recursos de Aprendizaje

### Rust para Robótica
- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Async Rust](https://rust-lang.github.io/async-book/)

### Librerías Utilizadas
- [Tokio](https://tokio.rs/) - Runtime async
- [Serde](https://serde.rs/) - Serialización
- [Anyhow](https://docs.rs/anyhow/) - Manejo de errores
- [Log](https://docs.rs/log/) - Logging

---

**¡Feliz desarrollo!** 🦀
Para cualquier duda, consulta los issues en GitHub o únete al Discord.
