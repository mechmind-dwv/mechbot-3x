use anyhow::Result;
use tokio::sync::broadcast;

#[derive(Debug)]
pub struct WebSocketServer {
    port: u16,
    clients: Vec<Client>,
    message_sender: broadcast::Sender<String>,
}

impl WebSocketServer {
    pub fn new(port: u16) -> Self {
        let (tx, _) = broadcast::channel(100);
        Self {
            port,
            clients: Vec::new(),
            message_sender: tx,
        }
    }

    pub async fn broadcast_telemetry(&self, data: &str) -> Result<()> {
        let _ = self.message_sender.send(data.to_string());
        Ok(())
    }

    pub fn add_client(&mut self, client: Client) {
        self.clients.push(client);
        println!(
            "🔗 Cliente WebSocket conectado. Total: {}",
            self.clients.len()
        );
    }

    pub fn remove_client(&mut self, client_id: u32) {
        self.clients.retain(|c| c.id != client_id);
        println!(
            "🔌 Cliente WebSocket desconectado. Total: {}",
            self.clients.len()
        );
    }

    pub fn get_client_count(&self) -> usize {
        self.clients.len()
    }
}

impl Default for WebSocketServer {
    fn default() -> Self {
        Self::new(8081)
    }
}

#[derive(Debug, Clone)]
pub struct Client {
    pub id: u32,
    pub address: String,
    pub connected_at: std::time::SystemTime,
}

impl Client {
    pub fn new(id: u32, address: String) -> Self {
        Self {
            id,
            address,
            connected_at: std::time::SystemTime::now(),
        }
    }
}

pub async fn start_server(port: u16) -> Result<()> {
    println!("🔌 Servidor WebSocket iniciado en puerto {}", port);
    // En implementación real usaríamos tokio-tungstenite
    Ok(())
}
