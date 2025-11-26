use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug)]
pub struct RestApi {
    robot_position: (f64, f64),
    robot_map: HashMap<String, String>,
}

impl RestApi {
    pub fn new() -> Self {
        Self {
            robot_position: (0.0, 0.0),
            robot_map: HashMap::from([
                ("type".to_string(), "occupancy_grid".to_string()),
                ("resolution".to_string(), "0.1".to_string()),
            ]),
        }
    }

    pub async fn handle_request(&self, method: &str, path: &str, body: &str) -> Result<String> {
        let key = format!("{} {}", method, path);

        match key.as_str() {
            "GET /api/v1/status" => Ok(self.handle_status()),
            "POST /api/v1/move" => Ok(self.handle_move(body)),
            "GET /api/v1/map" => Ok(self.handle_map()),
            "GET /api/v1/sensors" => Ok(self.handle_sensors()),
            _ => Ok(self.json_response(404, "Ruta no encontrada")),
        }
    }

    fn handle_status(&self) -> String {
        self.json_response(
            200,
            &format!(
                r#"{{"status": "operational", "position": {:?}, "battery": 85}}"#,
                self.robot_position
            ),
        )
    }

    fn handle_move(&self, body: &str) -> String {
        println!("🎯 Comando de movimiento recibido: {}", body);
        // En una implementación real, aquí se procesaría el JSON
        self.json_response(200, r#"{"message": "Movimiento ejecutado", "speed": 1.5}"#)
    }

    fn handle_map(&self) -> String {
        self.json_response(200, &format!(r#"{{"map": {:?}}}"#, self.robot_map))
    }

    fn handle_sensors(&self) -> String {
        self.json_response(200, r#"{"sensors": ["lidar", "camera", "imu", "gps"]}"#)
    }

    fn json_response(&self, status: u16, data: &str) -> String {
        format!(r#"{{"status": {}, "data": {}}}"#, status, data)
    }
}

impl Default for RestApi {
    fn default() -> Self {
        Self::new()
    }
}

pub async fn start_server(port: u16) -> Result<()> {
    println!("🌐 Servidor REST iniciado en puerto {}", port);
    println!("📡 Endpoints disponibles:");
    println!("   GET  /api/v1/status   - Estado del robot");
    println!("   POST /api/v1/move     - Mover a coordenadas");
    println!("   GET  /api/v1/map      - Mapa actual");
    println!("   GET  /api/v1/sensors  - Datos de sensores");
    Ok(())
}
