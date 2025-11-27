use super::Frame;
use anyhow::Result;

#[derive(Debug)]
pub struct FrameProcessor;

impl FrameProcessor {
    pub fn new(_config: &super::super::ProcessingConfig) -> Self {
        Self
    }

    pub fn preprocess_frame(&self, data: &[u8], width: u32, height: u32) -> Result<Frame> {
        Ok(Frame::new(
            data.to_vec(),
            width,
            height,
            3,
            super::FrameFormat::RGB,
        ))
    }
}
