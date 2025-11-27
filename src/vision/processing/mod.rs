pub mod frame_processor;
pub mod image_utils;

pub use frame_processor::FrameProcessor;
pub use image_utils::ImageUtils;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Frame {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub channels: u8,
    pub format: FrameFormat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FrameFormat {
    RGB,
    BGR,
    RGBA,
    GRAY,
    JPEG,
    PNG,
}

impl Frame {
    pub fn new(data: Vec<u8>, width: u32, height: u32, channels: u8, format: FrameFormat) -> Self {
        Self {
            data,
            width,
            height,
            channels,
            format,
        }
    }
}
