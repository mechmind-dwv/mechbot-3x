pub mod navigation_test;
pub mod robot_test;

// Re-export para fácil acceso
pub use navigation_test::*;
pub use robot_test::*;

/// Configuración para tests de integración
pub async fn setup_integration_test() -> mechbot_3x::Config {
    // Configuración específica para testing
    mechbot_3x::Config::default()
}

/// Limpiar recursos después de tests de integración
pub async fn teardown_integration_test() {
    // Limpiar archivos temporales, detener servicios, etc.
}
