use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentationMask {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u32>,
    pub confidence: Vec<f32>,
}

impl SegmentationMask {
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        Self {
            width,
            height,
            data: vec![0; size],
            confidence: vec![0.0; size],
        }
    }

    pub fn get_class_distribution(&self) -> Vec<ClassDistribution> {
        Vec::new() // Implementación simple
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentClass {
    pub id: u32,
    pub name: String,
    pub color: [u8; 3],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorMap;

impl ColorMap {
    pub fn get_color(&self, _class_id: u32) -> [u8; 3] {
        [0, 0, 0]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassDistribution {
    pub class_id: u32,
    pub pixel_count: usize,
    pub percentage: f32,
}
