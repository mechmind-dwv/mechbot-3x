use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Detection {
    pub bbox: BoundingBox,
    pub class: ObjectClass,
    pub confidence: f32,
    pub track_id: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundingBox {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl BoundingBox {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }

    pub fn iou(&self, _other: &BoundingBox) -> f32 {
        0.0 // Implementación simple
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectClass {
    pub id: u32,
    pub name: String,
    pub color: [u8; 3],
}

impl ObjectClass {
    pub fn new(id: u32, name: &str, color: [u8; 3]) -> Self {
        Self {
            id,
            name: name.to_string(),
            color,
        }
    }
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct VisionResult {
    pub detections: Vec<Detection>,
    pub segmentation: Option<super::SegmentationMask>,
    pub frame_metrics: FrameMetrics,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct FrameMetrics {
    pub detection_time_ms: f64,
    pub segmentation_time_ms: f64,
    pub total_objects: usize,
    pub frame_timestamp: u64,
}
