// Ejemplo de cliente WebSocket para MechBot-3X
// Ejecutar con: cargo run --example websocket_client

use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use tokio_tungstenite::{connect_async, tungstenite::Message};

#[tokio::main]
async fn main() {
    println!("🔌 Conectando al WebSocket de MechBot-3X...");

    let url = "ws://localhost:8081/ws";
    match connect_async(url).await {
        Ok((ws_stream, _)) => {
            println!("✅ Conectado a {}", url);

            let (mut write, mut read) = ws_stream.split();

            // Enviar comando de movimiento
            let move_cmd = json!({
                "type": "move_command",
                "id": "test_cmd_001",
                "linear": {"x": 0.5, "y": 0.0, "z": 0.0},
                "angular": {"x": 0.0, "y": 0.0, "z": 0.2},
                "timestamp": chrono::Utc::now().timestamp()
            });

            if write
                .send(Message::Text(move_cmd.to_string()))
                .await
                .is_ok()
            {
                println!("📤 Comando de movimiento enviado");
            }

            // Configurar telemetría
            let telemetry_config = json!({
                "type": "telemetry_config",
                "update_rate_hz": 10,
                "sensors": ["lidar", "imu", "pose"],
                "compression": true
            });

            if write
                .send(Message::Text(telemetry_config.to_string()))
                .await
                .is_ok()
            {
                println!("⚙️ Configuración de telemetría enviada");
            }

            // Recibir mensajes por 10 segundos
            let timeout = tokio::time::sleep(tokio::time::Duration::from_secs(10));
            tokio::pin!(timeout);

            loop {
                tokio::select! {
                    Some(Ok(message)) = read.next() => {
                        if let Message::Text(text) = message {
                            println!("📥 Mensaje recibido: {}", text);

                            // Parsear y mostrar el tipo de mensaje
                            if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&text) {
                                if let Some(msg_type) = parsed.get("type").and_then(|t| t.as_str()) {
                                    match msg_type {
                                        "telemetry" => println!("📊 Datos de telemetría recibidos"),
                                        "system_status" => println!("🖥️ Estado del sistema recibido"),
                                        "log_message" => println!("📝 Log recibido"),
                                        "command_ack" => println!("✅ Confirmación de comando recibida"),
                                        _ => println!("❓ Tipo de mensaje desconocido: {}", msg_type),
                                    }
                                }
                            }
                        }
                    }
                    _ = &mut timeout => {
                        println!("⏰ Timeout alcanzado. Desconectando...");
                        break;
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Error conectando al WebSocket: {}", e);
            eprintln!("💡 Asegúrate de que el servidor MechBot-3X esté ejecutándose");
        }
    }

    println!("👋 Cliente WebSocket finalizado");
}
