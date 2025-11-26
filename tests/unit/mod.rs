pub mod sensors_test;
pub mod vision_test;

// Re-export para fácil acceso
pub use sensors_test::*;
pub use vision_test::*;

/// Configuración común para todos los tests unitarios
pub fn setup_test_environment() {
    // Configurar variables de entorno para testing
    std::env::set_var("RUST_LOG", "debug");

    // Inicializar logger solo una vez
    let _ = env_logger::builder().is_test(true).try_init();
}

/// Limpiar después de los tests
pub fn teardown_test_environment() {
    // Limpiar archivos temporales si es necesario
}
