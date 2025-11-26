# Guía de Contribución - MechBot-3x

¡Gracias por tu interés en contribuir a MechBot-3x! Esta guía te ayudará a participar en el proyecto.

## 🎯 Cómo Contribuir

### Reportar Bugs
1. Verifica que no exista ya el issue en [GitHub Issues](https://github.com/mechmind-dwv/mechbot-3x/issues)
2. Usa la plantilla de bug report
3. Incluye logs, versiones y pasos para reproducir

### Sugerir Mejoras
1. Describe claramente la funcionalidad nueva
2. Explica el caso de uso
3. Propón una implementación si es posible

### Pull Requests
1. **Fork** el repositorio
2. **Crea una rama**: `git checkout -b feature/nueva-funcionalidad`
3. **Commit**: `git commit -m '✨ Agrega nueva funcionalidad'`
4. **Push**: `git push origin feature/nueva-funcionalidad`
5. **Abre un PR**

## 🛠️ Configuración de Desarrollo

### Requisitos
- Rust 1.81+
- Git
- Python 3.8+ (para algunos bindings)

### Primeros Pasos
```bash
# Clonar y configurar
git clone https://github.com/mechmind-dwv/mechbot-3x.git
cd mechbot-3x

# Compilar
cargo build

# Ejecutar tests
cargo test
```

## 📝 Estándares de Código

### Rust
```bash
# Formatear código
cargo fmt

# Linter
cargo clippy -- -D warnings

# Tests
cargo test
```

### Commits
Usamos Conventional Commits:
- `feat:` Nueva funcionalidad
- `fix:` Corrección de bugs
- `docs:` Documentación
- `style:` Formato, puntos y coma faltantes
- `refactor:` Cambios de código que no corrigen bugs
- `test:` Tests
- `chore:` Mantenimiento

## 🧪 Testing

### Ejecutar Tests
```bash
# Todos los tests
cargo test

# Tests específicos
cargo test test_nombre

# Tests con output
cargo test -- --nocapture
```

### Coverage
```bash
cargo tarpaulin --ignore-tests
```

## 📖 Documentación

### Generar Documentación
```bash
cargo doc --open
```

### Escribir Docs
- Usa comentarios Rustdoc `///`
- Documenta todas las funciones públicas
- Incluye ejemplos de uso

## 🐛 Debugging

### Logs
El proyecto usa logging estructurado:
```rust
log::info!("Mensaje informativo");
log::error!("Error específico");
```

### Performance
```bash
cargo bench
```

## 🤝 Código de Conducta

Respetamos a todos los contribuidores. Por favor:
- Sé amable y profesional
- Respeta diferentes puntos de vista
- Ayuda a mantener un ambiente inclusivo

## ❓ ¿Necesitas Ayuda?

- 📧 Email: ia.mechmind@gmail.com
- 💬 Discord: [MechMind Community](https://discord.gg/mechmind)
- 🐛 Issues: [GitHub Issues](https://github.com/mechmind-dwv/mechbot-3x/issues)

---

¡Gracias por hacer MechBot-3x mejor! 🚀
