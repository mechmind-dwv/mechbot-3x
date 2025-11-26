#[derive(Debug)]
pub struct ApiServer {
    port: u16,
    is_running: bool,
}

impl ApiServer {
    pub fn new(port: u16) -> Self {
        Self {
            port,
            is_running: false,
        }
    }
    
    pub fn is_running(&self) -> bool {
        self.is_running
    }
    
    pub fn stop(&mut self) {
        self.is_running = false;
        println!("API Server stopped");
    }
}

#[derive(Debug)]
pub struct WebSocketServer {
    port: u16,
}

impl WebSocketServer {
    pub fn new(port: u16) -> Self {
        Self { port }
    }
}
