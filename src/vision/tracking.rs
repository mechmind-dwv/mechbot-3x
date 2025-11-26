use anyhow::Result;

#[derive(Debug)]
pub struct ObjectTracker {
    tracks: Vec<Track>,
    next_track_id: u32,
}

impl ObjectTracker {
    pub fn new() -> Self {
        Self {
            tracks: Vec::new(),
            next_track_id: 1,
        }
    }

    pub fn update(&mut self, detections: &[(f64, f64, f64, f64)]) -> Result<Vec<Track>> {
        // Algoritmo de tracking simple (en implementación real usaríamos Kalman filter)
        for &(x, y, w, h) in detections {
            let center = (x + w / 2.0, y + h / 2.0);

            // Buscar track existente más cercano
            if let Some(track) = self.tracks.iter_mut().find(|t| {
                let track_center = (t.bbox.0 + t.bbox.2 / 2.0, t.bbox.1 + t.bbox.3 / 2.0);
                distance(center, track_center) < 50.0 // Umbral de distancia
            }) {
                // Actualizar track existente
                track.bbox = (x, y, w, h);
                track.age += 1;
            } else {
                // Crear nuevo track
                self.tracks.push(Track {
                    id: self.next_track_id,
                    bbox: (x, y, w, h),
                    age: 1,
                    class: "unknown".to_string(),
                });
                self.next_track_id += 1;
            }
        }

        // Eliminar tracks viejos
        self.tracks.retain(|track| track.age < 10);

        Ok(self.tracks.clone())
    }

    pub fn get_active_tracks(&self) -> &[Track] {
        &self.tracks
    }
}

impl Default for ObjectTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct Track {
    pub id: u32,
    pub bbox: (f64, f64, f64, f64),
    pub age: u32,
    pub class: String,
}

fn distance(a: (f64, f64), b: (f64, f64)) -> f64 {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    (dx * dx + dy * dy).sqrt()
}
