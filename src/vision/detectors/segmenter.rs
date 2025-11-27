use super::super::models::{SegmentationMask, SegmentClass, ColorMap};
use super::super::processing::Frame;
use anyhow::Result;
use ort::{Session, Value};

#[derive(Debug)]
pub struct SemanticSegmenter {
    session: Session,
    classes: Vec<SegmentClass>,
    color_map: ColorMap,
    input_width: u32,
    input_height: u32,
}

impl SemanticSegmenter {
    pub async fn new(config: &super::super::SegmentationConfig) -> Result<Self> {
        log::info!("🎨 Inicializando segmentador semántico...");
        
        // Cargar modelo ONNX
        let session = Session::builder()?
            .with_optimization_level(ort::GraphOptimizationLevel::All)?
            .with_intra_threads(4)?
            .commit_from_file(&config.model_path)?;

        // Crear clases y mapa de colores
        let classes: Vec<SegmentClass> = config.classes.iter()
            .enumerate()
            .map(|(i, name)| SegmentClass {
                id: i as u32,
                name: name.clone(),
                color: *config.colors.get(i).unwrap_or(&[0, 0, 0]),
            })
            .collect();

        let color_map = ColorMap::new(classes.clone());

        log::info!("✅ Segmentador inicializado: {} clases", classes.len());
        
        Ok(Self {
            session,
            classes,
            color_map,
            input_width: 512, // Típico para modelos de segmentación
            input_height: 512,
        })
    }

    pub async fn segment_frame(&self, frame: &Frame) -> Result<SegmentationMask> {
        // Preprocesar frame para segmentación
        let input_tensor = self.preprocess_frame(frame)?;
        
        // Ejecutar inferencia
        let outputs = self.session.run(vec![input_tensor])?;
        let segmentation_mask = self.postprocess_outputs(outputs, frame.width, frame.height)?;
        
        Ok(segmentation_mask)
    }

    fn preprocess_frame(&self, frame: &Frame) -> Result<Value> {
        // Convertir frame a tensor normalizado para segmentación
        let mut input_data = vec![0.0f32; (3 * self.input_width * self.input_height) as usize];
        
        // Preprocesamiento real:
        // - Redimensionar a input_width x input_height
        // - Normalizar según el modelo
        // - Convertir BGR a RGB si es necesario
        
        // Simulación
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

    fn postprocess_outputs(&self, outputs: Vec<Value>, original_width: u32, original_height: u32) -> Result<SegmentationMask> {
        let mut mask = SegmentationMask::new(original_width, original_height);

        // Simulación de segmentación para desarrollo
        // En producción, aquí se procesaría la salida real del modelo
        if cfg!(debug_assertions) {
            // Crear un patrón simple de segmentación
            for y in 0..original_height {
                for x in 0..original_width {
                    let class_id = if x < original_width / 2 {
                        0 // background
                    } else if y < original_height / 2 {
                        1 // person
                    } else {
                        2 // car
                    };
                    
                    let confidence = 0.8 + (x as f32 % 10.0) * 0.02; // Variar confianza
                    mask.set_class_at(x, y, class_id, confidence);
                }
            }
        }

        Ok(mask)
    }

    pub fn get_color_map(&self) -> &ColorMap {
        &self.color_map
    }

    pub fn get_classes(&self) -> &[SegmentClass] {
        &self.classes
    }

    pub fn create_overlay(&self, frame: &Frame, mask: &SegmentationMask, alpha: f32) -> Vec<u8> {
        let mut overlay = frame.data.clone();
        let colored_mask = mask.to_colored_image(&self.color_map);

        // Mezclar frame original con máscara coloreada
        for i in (0..overlay.len()).step_by(3) {
            if i + 2 < colored_mask.len() {
                let r_orig = overlay[i] as f32;
                let g_orig = overlay[i + 1] as f32;
                let b_orig = overlay[i + 2] as f32;

                let r_mask = colored_mask[i] as f32;
                let g_mask = colored_mask[i + 1] as f32;
                let b_mask = colored_mask[i + 2] as f32;

                overlay[i] = (r_orig * (1.0 - alpha) + r_mask * alpha) as u8;
                overlay[i + 1] = (g_orig * (1.0 - alpha) + g_mask * alpha) as u8;
                overlay[i + 2] = (b_orig * (1.0 - alpha) + b_mask * alpha) as u8;
            }
        }

        overlay
    }
}
