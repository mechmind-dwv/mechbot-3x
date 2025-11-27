use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::Response,
};
use futures::{SinkExt, StreamExt};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{interval, Duration};

use super::AppState;

type SharedState = Arc<RwLock<AppState>>;

pub async fn start_websocket_server(port: u16) -> anyhow::Result<()> {
    let state = Arc::new(RwLock::new(AppState::default()));

    let app = axum::Router::new()
        .route("/telemetry", axum::routing::get(websocket_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;

    log::info!("📡 Servidor WebSocket iniciado en ws://localhost:{}", port);

    axum::serve(listener, app).await?;

    Ok(())
}

async fn websocket_handler(ws: WebSocketUpgrade, State(state): State<SharedState>) -> Response {
    ws.on_upgrade(|socket| handle_websocket_connection(socket, state))
}

async fn handle_websocket_connection(socket: WebSocket, state: SharedState) {
    let (mut sender, mut receiver) = socket.split();

    log::info!("🔌 Nueva conexión WebSocket establecida");

    // Canal para enviar telemetría
    let mut telemetry_interval = interval(Duration::from_millis(100)); // 10 Hz

    // Manejar mensajes entrantes y enviar telemetría
    let mut sequence_number = 0;

    loop {
        tokio::select! {
            // Enviar telemetría periódicamente
            _ = telemetry_interval.tick() => {
                sequence_number += 1;

                let telemetry_data = generate_telemetry_data(&state, sequence_number).await;

                if let Ok(json_data) = serde_json::to_string(&telemetry_data) {
                    if sender.send(Message::Text(json_data)).await.is_err() {
                        break;
                    }
                }
            }

            // Manejar mensajes entrantes
            message = receiver.next() => {
                match message {
                    Some(Ok(Message::Text(text))) => {
                        handle_websocket_message(&text, &state).await;
                    }
                    Some(Ok(Message::Close(_))) => {
                        break;
                    }
                    Some(Err(_)) => {
                        break;
                    }
                    None => {
                        break;
                    }
                    _ => {}
                }
            }
        }
    }

    log::info!("🔌 Conexión WebSocket cerrada");
}

async fn generate_telemetry_data(state: &SharedState, sequence: u64) -> serde_json::Value {
    let state = state.read().await;

    // Simular datos de sensores en tiempo real
    let simulated_position = json!({
        "x": state.robot_status.position.x + (sequence as f64 * 0.01).sin() * 0.1,
        "y": state.robot_status.position.y + (sequence as f64 * 0.02).cos() * 0.1,
        "theta": (sequence as f64 * 0.05).sin() * 0.5
    });

    let simulated_sensors = json!({
        "lidar": {
            "points": (0..360).map(|i| {
                json!({
                    "angle": i,
                    "distance": 2.0 + (i as f64 * 0.01745).sin().abs() * 1.5
                })
            }).collect::<Vec<_>>(),
            "min_distance": 0.15,
            "max_distance": 4.0
        },
        "imu": {
            "acceleration": {
                "x": (sequence as f64 * 0.1).sin() * 0.1,
                "y": (sequence as f64 * 0.15).cos() * 0.1,
                "z": 9.81
            },
            "gyroscope": {
                "x": (sequence as f64 * 0.05).sin() * 0.01,
                "y": (sequence as f64 * 0.07).cos() * 0.01,
                "z": (sequence as f64 * 0.03).sin() * 0.02
            }
        },
        "camera": {
            "objects_detected": (sequence % 5) as u32,
            "resolution": [640, 480]
        }
    });

    json!({
        "sequence": sequence,
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "position": simulated_position,
        "sensors": simulated_sensors,
        "battery_level": state.robot_status.battery_level - (sequence as f64 * 0.001),
        "state": state.robot_status.status.state,
        "uptime": sequence
    })
}

async fn handle_websocket_message(message: &str, state: &SharedState) {
    log::info!("📨 Mensaje WebSocket recibido: {}", message);

    // Aquí puedes procesar comandos específicos del WebSocket
    if let Ok(command) = serde_json::from_str::<serde_json::Value>(message) {
        if let Some(cmd_type) = command.get("type").and_then(|t| t.as_str()) {
            match cmd_type {
                "emergency_stop" => {
                    log::warn!("🛑 Comando de parada de emergencia recibido");
                    // Implementar lógica de parada de emergencia
                }
                "change_mode" => {
                    if let Some(mode) = command.get("mode").and_then(|m| m.as_str()) {
                        log::info!("🔄 Cambiando modo a: {}", mode);
                        // Actualizar estado del robot
                    }
                }
                _ => {
                    log::debug!("❓ Comando WebSocket desconocido: {}", cmd_type);
                }
            }
        }
    }
}
