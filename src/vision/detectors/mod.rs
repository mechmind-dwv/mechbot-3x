pub mod yolo;
pub mod segmenter;

pub use yolo::YOLODetector;
pub use segmenter::SemanticSegmenter;

use super::processing::Frame;
use anyhow::Result;

// Placeholder implementations para compilación
#[derive(Debug)]
pub struct YOLODetector;

impl YOLODetector {
    pub async fn new(_config: &super::YOLOConfig) -> Result<Self> {
        Ok(Self)
    }
    
    pub async fn detect_objects(&self, _frame: &Frame) -> Result<Vec<super::models::Detection>> {
        Ok(Vec::new())
    }
}

#[derive(Debug)]
pub struct SemanticSegmenter;

impl SemanticSegmenter {
    pub async fn new(_config: &super::SegmentationConfig) -> Result<Self> {
        Ok(Self)
    }
    
    pub async fn segment_frame(&self, _frame: &Frame) -> Result<super::models::SegmentationMask> {
        Ok(super::models::SegmentationMask::new(640, 480))
    }
}
