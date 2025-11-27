use mechbot_3x::{initialize_system, Config};
use tokio;

/// Test de integración del sistema completo
#[tokio::test]
async fn test_system_initialization() {
    let config = Config::default();

    let result = initialize_system(config).await;
    assert!(
        result.is_ok(),
        "El sistema debería inicializar correctamente"
    );

    let (lidar, camera, imu, navigation, vision, api_server) = result.unwrap();

    // Verificar que todos los componentes se crearon
    assert!(api_server.is_running());

    // Verificar características de visión
    let features = vision.get_available_features();
    assert!(!features.is_empty());
}

/// Test de configuración desde archivo
#[test]
fn test_config_loading() {
    let config = Config::default();

    assert_eq!(config.robot.name, "MechBot-3x");
    assert!(config.api.rest_port.is_some());
}

/// Test de serialización/deserialización
#[test]
fn test_config_serialization() {
    let config = Config::default();

    let serialized = serde_json::to_string(&config);
    assert!(serialized.is_ok());

    let deserialized: Result<Config, _> = serde_json::from_str(&serialized.unwrap());
    assert!(deserialized.is_ok());
}
