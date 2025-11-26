# Visión por Computadora - Detección de Objetos

## 🔍 Modelos Implementados

### YOLO (You Only Look Once)
```rust
pub struct YOLODetector {
    model: ONNXModel,
    classes: Vec<String>,
    confidence_threshold: f32,
}

impl YOLODetector {
    pub async fn detect_objects(&self, frame: &Frame) -> Result<Vec<Detection>>;
}
```

### Segmentación Semántica
```rust
pub struct SemanticSegmenter {
    model: TensorFlowModel,
    class_colors: HashMap<u32, RGB>,
}

impl SemanticSegmenter {
    pub fn segment_frame(&self, frame: &Frame) -> Result<SegmentationMask>;
}
```
