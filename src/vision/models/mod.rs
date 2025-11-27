mod detection;
mod segmentation;

pub use detection::{Detection, BoundingBox, ObjectClass, ClassProbability};
pub use segmentation::{SegmentationMask, SegmentClass, ColorMap};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}
