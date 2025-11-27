use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tokio::sync::RwLock;

use super::{AppState, MapData, MoveCommand, RobotStatus, SensorData};

type SharedState = Arc<RwLock<AppState>>;

pub async fn start_rest_server(port: u16) -> anyhow::Result<()> {
    let state = Arc::new(RwLock::new(AppState::default()));

    let app = Router::new()
        .route("/api/v1/status", get(get_status))
        .route("/api/v1/move", post(move_to_position))
        .route("/api/v1/map", get(get_map))
        .route("/api/v1/sensors", get(get_sensors))
        .route("/health", get(health_check))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;

    log::info!("🌐 Servidor REST iniciado en http://localhost:{}", port);

    axum::serve(listener, app).await?;

    Ok(())
}

// Handler para el estado del robot
async fn get_status(State(state): State<SharedState>) -> Json<RobotStatus> {
    let state = state.read().await;
    Json(state.robot_status.clone())
}

// Handler para mover el robot
async fn move_to_position(
    State(state): State<SharedState>,
    Json(command): Json<MoveCommand>,
) -> (StatusCode, Json<serde_json::Value>) {
    log::info!(
        "🎯 Comando de movimiento recibido: x={}, y={}, speed={}",
        command.x,
        command.y,
        command.speed
    );

    // Actualizar posición en el estado
    {
        let mut state = state.write().await;
        state.robot_status.position.x = command.x;
        state.robot_status.position.y = command.y;
        state.robot_status.position.timestamp = chrono::Utc::now().to_rfc3339();
    }

    let response = serde_json::json!({
        "status": "success",
        "message": "Movimiento iniciado",
        "target": {
            "x": command.x,
            "y": command.y
        },
        "speed": command.speed
    });

    (StatusCode::OK, Json(response))
}

// Handler para obtener el mapa
async fn get_map(State(state): State<SharedState>) -> Json<MapData> {
    let state = state.read().await;
    Json(state.map_data.clone())
}

// Handler para datos de sensores
async fn get_sensors(State(state): State<SharedState>) -> Json<SensorData> {
    let state = state.read().await;
    Json(state.last_sensor_data.clone())
}

// Health check endpoint
async fn health_check() -> (StatusCode, Json<serde_json::Value>) {
    let response = serde_json::json!({
        "status": "healthy",
        "service": "mechbot-3x",
        "timestamp": chrono::Utc::now().to_rfc3339()
    });

    (StatusCode::OK, Json(response))
}
