// Módulo de visión - Por implementar
pub struct VisionProcessor;

impl VisionProcessor {
    pub fn new() -> Self {
        Self
    }
    
    pub fn detect_objects(&self) -> Result<Vec<String>, anyhow::Error> {
        Ok(vec![])
    }
}
