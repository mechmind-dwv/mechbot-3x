use super::super::models::{Detection, BoundingBox, ObjectClass};
use super::super::processing::Frame;
use anyhow::Result;
use ort::{Session, Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug)]
pub struct YOLODetector {
    session: Session,
    classes: HashMap<u32, ObjectClass>,
    confidence_threshold: f32,
    nms_threshold: f32,
    input_width: u32,
    input_height: u32,
}

impl YOLODetector {
    pub async fn new(config: &super::super::YOLOConfig) -> Result<Self> {
        log::info!("🚀 Inicializando detector YOLO...");
        
        // Cargar modelo ONNX
        let session = Session::builder()?
            .with_optimization_level(ort::GraphOptimizationLevel::All)?
            .with_intra_threads(4)?
            .commit_from_file(&config.model_path)?;

        // Cargar clases COCO
        let classes = super::super::models::detection::get_coco_classes();

        log::info!("✅ YOLO inicializado: {} clases", classes.len());
        
        Ok(Self {
            session,
            classes,
            confidence_threshold: config.confidence_threshold,
            nms_threshold: config.nms_threshold,
            input_width: config.input_width,
            input_height: config.input_height,
        })
    }

    pub async fn detect_objects(&self, frame: &Frame) -> Result<Vec<Detection>> {
        // Preprocesar frame para YOLO
        let input_tensor = self.preprocess_frame(frame)?;
        
        // Ejecutar inferencia
        let outputs = self.session.run(vec![input_tensor])?;
        let detections = self.postprocess_outputs(outputs, frame.width, frame.height)?;
        
        Ok(detections)
    }

    fn preprocess_frame(&self, frame: &Frame) -> Result<Value> {
        // Convertir frame a tensor normalizado para YOLO
        let mut input_data = vec![0.0f32; (3 * self.input_width * self.input_height) as usize];
        
        // Aquí iría el preprocesamiento real:
        // - Redimensionar a input_width x input_height
        // - Normalizar valores de píxel
        // - Convertir BGR a RGB
        // - Transponer a formato CHW
        
        // Por ahora, simulación
        for i in 0..input_data.len() {
            input_data[i] = (i % 255) as f32 / 255.0;
        }

        let input_tensor = Value::from_array(
            self.session.allocator(),
            &ndarray::Array4::from_shape_vec(
                (1, 3, self.input_height as usize, self.input_width as usize),
                input_data,
            )?,
        )?;

        Ok(input_tensor)
    }

    fn postprocess_outputs(&self, outputs: Vec<Value>, original_width: u32, original_height: u32) -> Result<Vec<Detection>> {
        let mut detections = Vec::new();

        // Simulación de detecciones para desarrollo
        // En producción, aquí se procesaría la salida real de YOLO
        if cfg!(debug_assertions) {
            detections.push(Detection {
                bbox: BoundingBox::new(100.0, 100.0, 200.0, 300.0),
                class: self.classes[&0].clone(), // persona
                confidence: 0.85,
                track_id: Some(1),
            });

            detections.push(Detection {
                bbox: BoundingBox::new(400.0, 150.0, 180.0, 250.0),
                class: self.classes[&2].clone(), // car
                confidence: 0.92,
                track_id: Some(2),
            });
        }

        // Aplicar Non-Maximum Suppression
        let filtered_detections = self.apply_nms(detections);
        
        Ok(filtered_detections)
    }

    fn apply_nms(&self, mut detections: Vec<Detection>) -> Vec<Detection> {
        // Ordenar por confianza descendente
        detections.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());

        let mut filtered = Vec::new();
        let mut suppressed = vec![false; detections.len()];

        for i in 0..detections.len() {
            if suppressed[i] {
                continue;
            }

            filtered.push(detections[i].clone());

            for j in (i + 1)..detections.len() {
                if suppressed[j] {
                    continue;
                }

                let iou = detections[i].bbox.iou(&detections[j].bbox);
                if iou > self.nms_threshold {
                    suppressed[j] = true;
                }
            }
        }

        filtered
    }

    pub fn get_classes(&self) -> &HashMap<u32, ObjectClass> {
        &self.classes
    }

    pub fn get_detection_stats(&self, detections: &[Detection]) -> DetectionStats {
        let mut stats = DetectionStats::default();
        
        for detection in detections {
            stats.total_detections += 1;
            stats.confidence_sum += detection.confidence;
            
            *stats.class_counts.entry(detection.class.id).or_insert(0) += 1;
        }

        if stats.total_detections > 0 {
            stats.average_confidence = stats.confidence_sum / stats.total_detections as f32;
        }

        stats
    }
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct DetectionStats {
    pub total_detections: usize,
    pub average_confidence: f32,
    pub confidence_sum: f32,
    pub class_counts: HashMap<u32, u32>,
}
