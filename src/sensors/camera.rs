use super::{Sensor, SensorStatus};
use anyhow::Result;

#[derive(Debug)]
pub struct Camera {
    device_id: u32,
    resolution: (u32, u32),
    fps: u32,
    status: SensorStatus,
}

impl Camera {
    pub fn new(device_id: u32, resolution: (u32, u32), fps: u32) -> Self {
        Self {
            device_id,
            resolution,
            fps,
            status: SensorStatus {
                connected: false,
                last_update: std::time::SystemTime::now(),
                error_count: 0,
            },
        }
    }

    pub fn capture_frame(&mut self) -> Result<Frame> {
        // Simular captura de frame
        let frame = Frame {
            data: vec![0; (self.resolution.0 * self.resolution.1 * 3) as usize], // RGB
            width: self.resolution.0,
            height: self.resolution.1,
            timestamp: std::time::SystemTime::now(),
        };
        self.status.last_update = std::time::SystemTime::now();
        Ok(frame)
    }

    pub fn set_resolution(&mut self, width: u32, height: u32) {
        self.resolution = (width, height);
    }

    pub fn set_fps(&mut self, fps: u32) {
        self.fps = fps;
    }
}

impl Sensor for Camera {
    fn initialize(&mut self) -> Result<()> {
        self.status.connected = true;
        println!(
            "📷 Cámara {} inicializada - Resolución: {}x{} @ {} FPS",
            self.device_id, self.resolution.0, self.resolution.1, self.fps
        );
        Ok(())
    }

    fn read_data(&mut self) -> Result<Vec<u8>> {
        let frame = self.capture_frame()?;
        Ok(frame.data)
    }

    fn get_status(&self) -> SensorStatus {
        self.status.clone()
    }
}

#[derive(Debug)]
pub struct Frame {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub timestamp: std::time::SystemTime,
}
