# 🤝 Guía de Contribución

¡Gracias por tu interés en contribuir a MechBot-3x! Somos una comunidad abierta que valora todas las contribuciones.

## 🎯 ¿Cómo contribuir?

### 1. Reportar Bugs
- Usa el template de bug report
- Incluye logs, versiones y pasos para reproducir
- Verifica que no sea un duplicado

### 2. Sugerir Mejoras
- Describe claramente la funcionalidad
- Explica el caso de uso
- Propón una solución si es posible

### 3. Enviar Código
1. **Fork** el repositorio
2. **Crea una rama** descriptiva:
   ```bash
   git checkout -b feature/nueva-funcionalidad
   # o
   git checkout -b fix/correccion-bug
   ```
3. **Sigue las convenciones** de código
4. **Ejecuta los tests**
5. **Envía un Pull Request**

## 🔧 Convenciones de Código

### Estilo de Código Rust
```bash
# Formatear código
cargo fmt

# Linter
cargo clippy -- -D warnings

# Tests
cargo test

# Documentación
cargo doc --open
```

### Estructura de Commits
Usamos [Conventional Commits](https://www.conventionalcommits.org/):

- `✨ feat:` Nueva funcionalidad
- `🐛 fix:` Corrección de bug
- `📚 docs:` Documentación
- `🎨 style:` Formato, punto y coma faltante, etc.
- `🔧 refactor:` Refactorización de código
- `✅ test:` Agregar o corregir tests
- `⚡ perf:` Mejora de rendimiento
- `🧹 chore:` Cambios en build, herramientas, etc.

### Estándares de Código
- **Documenta** todas las funciones públicas
- **Escribe tests** para nueva funcionalidad
- **Mantén** la cobertura de tests > 80%
- **Usa tipos fuertes** y evita `unwrap()` sin contexto
- **Sigue** los principios de Rust (ownership, borrowing)

## 🧪 Testing

```bash
# Todos los tests
cargo test

# Tests específicos
cargo test test_nombre_del_test

# Tests con output verbose
cargo test -- --nocapture

# Benchmarks
cargo bench

# Coverage (instalar cargo-tarpaulin)
cargo tarpaulin --ignore-tests
```

## 📝 Pull Request Process

1. **Actualiza** tu rama con `main`
2. **Asegura** que todos los tests pasen
3. **Actualiza** la documentación si es necesario
4. **Agrega** una descripción clara del PR
5. **Menciona** issues relacionados
6. **Espera** review del equipo

## 🏷️ Etiquetas de Issues

- `good first issue` - Ideal para nuevos contribuidores
- `help wanted` - Necesita asistencia
- `bug` - Error o problema
- `enhancement` - Mejora de funcionalidad
- `documentation` - Mejora de docs
- `question` - Pregunta o duda

## 📞 Soporte

- **Discord:** [Únete a nuestro servidor](https://discord.gg/mechmind)
- **Email:** ia.mechmind@gmail.com
- **Issues:** [GitHub Issues](https://github.com/mechmind-dwv/mechbot-3x/issues)

## 📜 Licencia

Al contribuir, aceptas que tus contribuciones serán licenciadas bajo la [Licencia MIT](LICENSE).

---

¡Gracias por hacer de MechBot-3x un proyecto mejor! 🚀
