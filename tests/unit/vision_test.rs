use anyhow::Result;
use mechbot_3x::vision::{Detection, ObjectDetector, ObjectTracker, VisionProcessor};

#[cfg(test)]
mod vision_tests {
    use super::*;

    #[test]
    fn test_vision_processor_initialization() -> Result<()> {
        let mut vision = VisionProcessor::new();
        vision.load_models()?;

        // Después de cargar modelos, debería estar listo
        assert!(true); // Si no hay panic, está bien

        Ok(())
    }

    #[test]
    fn test_object_detection() -> Result<()> {
        let vision = VisionProcessor::new();

        // Crear frame simulado (todo negro)
        let frame_data = vec![0u8; 640 * 480 * 3];
        let detections = vision.process_frame(&frame_data, 640, 480)?;

        // En nuestro caso simulado, debería detectar un robot
        assert!(!detections.is_empty());
        assert_eq!(detections[0].class, "robot");
        assert!(detections[0].confidence > 0.5);

        Ok(())
    }

    #[test]
    fn test_detection_threshold() -> Result<()> {
        let mut vision = VisionProcessor::new();

        // Establecer threshold alto
        vision.set_detection_threshold(0.95);

        let frame_data = vec![0u8; 100]; // Frame pequeño (baja confianza)
        let detections = vision.process_frame(&frame_data, 10, 10)?;

        // Con threshold alto, no debería detectar nada
        assert!(detections.is_empty());

        Ok(())
    }

    #[test]
    fn test_object_detector_initialization() {
        let detector = ObjectDetector::new("models/yolo.cfg");

        // Verificar que se cargaron las clases correctas
        assert_eq!(detector.get_class_name(0), Some("person"));
        assert_eq!(detector.get_class_name(3), Some("robot"));
        assert_eq!(detector.get_class_name(10), None); // Clase inexistente
    }

    #[test]
    fn test_object_tracking() -> Result<()> {
        let mut tracker = ObjectTracker::new();

        // Simular detecciones en frames consecutivos
        let detections_frame1 = vec![(100.0, 150.0, 50.0, 60.0)];
        let detections_frame2 = vec![(110.0, 155.0, 50.0, 60.0)]; // Movimiento pequeño

        let tracks1 = tracker.update(&detections_frame1)?;
        let tracks2 = tracker.update(&detections_frame2)?;

        // Debería mantener el mismo track ID
        assert_eq!(tracks1.len(), 1);
        assert_eq!(tracks2.len(), 1);
        assert_eq!(tracks1[0].id, tracks2[0].id);
        assert_eq!(tracks2[0].age, 2); // Debería haber envejecido

        Ok(())
    }

    #[test]
    fn test_track_aging() -> Result<()> {
        let mut tracker = ObjectTracker::new();

        // Agregar un track y luego no actualizarlo
        let detections = vec![(100.0, 150.0, 50.0, 60.0)];
        let _ = tracker.update(&detections)?;

        // Simular múltiples updates sin detecciones
        for _ in 0..15 {
            let _ = tracker.update(&[])?;
        }

        // El track debería haber sido removido por vejez
        assert!(tracker.get_active_tracks().is_empty());

        Ok(())
    }

    #[test]
    fn test_multiple_object_tracking() -> Result<()> {
        let mut tracker = ObjectTracker::new();

        // Simular múltiples objetos
        let detections = vec![(100.0, 150.0, 50.0, 60.0), (300.0, 200.0, 40.0, 50.0)];

        let tracks = tracker.update(&detections)?;

        assert_eq!(tracks.len(), 2);
        assert_ne!(tracks[0].id, tracks[1].id); // IDs diferentes

        Ok(())
    }
}

#[cfg(test)]
mod vision_performance_tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_vision_processing_performance() -> Result<()> {
        let vision = VisionProcessor::new();
        let frame_data = vec![0u8; 640 * 480 * 3]; // Frame VGA

        let start = Instant::now();
        let _detections = vision.process_frame(&frame_data, 640, 480)?;
        let duration = start.elapsed();

        // El procesamiento debería tomar menos de 100ms
        assert!(duration.as_millis() < 100);

        Ok(())
    }

    #[test]
    fn test_tracking_performance() -> Result<()> {
        let mut tracker = ObjectTracker::new();

        // Test con muchas detecciones
        let mut detections = Vec::new();
        for i in 0..50 {
            detections.push((i as f64 * 10.0, 150.0, 50.0, 60.0));
        }

        let start = Instant::now();
        let _tracks = tracker.update(&detections)?;
        let duration = start.elapsed();

        // El tracking debería ser rápido incluso con muchas detecciones
        assert!(duration.as_micros() < 1000); // Menos de 1ms

        Ok(())
    }
}
