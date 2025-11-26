//! Sensores de salud para smartphones - Tu visión de salvar vidas
use anyhow::Result;
use serde::Serialize;
use std::collections::HashMap;

/// Representa los datos de salud de un usuario
#[derive(Debug, Serialize, Clone)]
pub struct HealthData {
    pub heart_rate: Option<u32>,        // latidos por minuto
    pub oxygen_saturation: Option<f64>, // SpO2 en porcentaje
    pub respiratory_rate: Option<u32>,  // respiraciones por minuto
    pub stress_level: Option<f64>,      // nivel de estrés 0.0-1.0
    pub activity_level: Option<f64>,    // nivel de actividad 0.0-1.0
    pub sleep_quality: Option<f64>,     // calidad de sueño 0.0-1.0
    pub timestamp: std::time::SystemTime,
}

/// Sensor virtual que simula los sensores de un smartphone
pub struct SmartphoneHealthSensor {
    user_id: String,
    sensor_readings: HashMap<String, f64>,
    baseline_data: HealthData,
}

impl SmartphoneHealthSensor {
    pub fn new(user_id: &str) -> Self {
        Self {
            user_id: user_id.to_string(),
            sensor_readings: HashMap::new(),
            baseline_data: HealthData {
                heart_rate: Some(72),
                oxygen_saturation: Some(98.5),
                respiratory_rate: Some(16),
                stress_level: Some(0.3),
                activity_level: Some(0.5),
                sleep_quality: Some(0.8),
                timestamp: std::time::SystemTime::now(),
            },
        }
    }

    /// Simula la lectura de todos los sensores del smartphone
    pub fn read_all_sensors(&mut self) -> Result<HealthData> {
        println!("📱 Leyendo sensores de salud del usuario: {}", self.user_id);

        // Simular lecturas de sensores con variaciones naturales
        let heart_rate = Some((65 + rand::random::<u32>() % 40) as u32); // 65-105 BPM
        let oxygen_saturation = Some(95.0 + (rand::random::<f64>() * 5.0)); // 95-100%
        let respiratory_rate = Some(12 + rand::random::<u32>() % 12); // 12-24 RPM
        let stress_level = Some(rand::random::<f64>() * 0.8); // 0.0-0.8
        let activity_level = Some(rand::random::<f64>()); // 0.0-1.0
        let sleep_quality = Some(0.5 + (rand::random::<f64>() * 0.5)); // 0.5-1.0

        let health_data = HealthData {
            heart_rate,
            oxygen_saturation,
            respiratory_rate,
            stress_level,
            activity_level,
            sleep_quality,
            timestamp: std::time::SystemTime::now(),
        };

        // Almacenar lecturas para análisis de tendencias
        self.store_sensor_readings(&health_data);

        Ok(health_data)
    }

    /// Analiza los datos de salud en busca de anomalías
    pub fn analyze_health_risks(&self, current_data: &HealthData) -> Vec<HealthAlert> {
        let mut alerts = Vec::new();

        // Análisis de frecuencia cardíaca
        if let (Some(current_hr), Some(baseline_hr)) =
            (current_data.heart_rate, self.baseline_data.heart_rate)
        {
            if current_hr > baseline_hr + 30 {
                alerts.push(HealthAlert::CriticalHeartRate(current_hr));
            } else if current_hr > baseline_hr + 15 {
                alerts.push(HealthAlert::HighHeartRate(current_hr));
            }
        }

        // Análisis de saturación de oxígeno
        if let (Some(current_spo2), Some(baseline_spo2)) = (
            current_data.oxygen_saturation,
            self.baseline_data.oxygen_saturation,
        ) {
            if current_spo2 < 90.0 {
                alerts.push(HealthAlert::CriticalOxygenLevel(current_spo2));
            } else if current_spo2 < baseline_spo2 - 5.0 {
                alerts.push(HealthAlert::LowOxygenLevel(current_spo2));
            }
        }

        // Análisis de nivel de estrés
        if let Some(stress) = current_data.stress_level {
            if stress > 0.8 {
                alerts.push(HealthAlert::HighStressLevel(stress));
            }
        }

        alerts
    }

    /// Predice posibles condiciones médicas basadas en patrones
    pub fn predict_conditions(&self, health_data: &HealthData) -> Vec<MedicalCondition> {
        let mut predictions = Vec::new();

        // Detección de posible taquicardia
        if let Some(hr) = health_data.heart_rate {
            if hr > 100 {
                predictions.push(MedicalCondition::PossibleTachycardia);
            }
        }

        // Detección de posible hipoxia
        if let Some(spo2) = health_data.oxygen_saturation {
            if spo2 < 92.0 {
                predictions.push(MedicalCondition::PossibleHypoxia);
            }
        }

        // Detección de posible estrés crónico
        if let Some(stress) = health_data.stress_level {
            if stress > 0.7 {
                predictions.push(MedicalCondition::PossibleChronicStress);
            }
        }

        predictions
    }

    fn store_sensor_readings(&mut self, health_data: &HealthData) {
        if let Some(hr) = health_data.heart_rate {
            self.sensor_readings
                .insert("heart_rate".to_string(), hr as f64);
        }
        if let Some(spo2) = health_data.oxygen_saturation {
            self.sensor_readings
                .insert("oxygen_saturation".to_string(), spo2);
        }
        // ... almacenar otros sensores
    }
}

/// Alertas de salud que pueden requerir atención
#[derive(Debug, Clone)]
pub enum HealthAlert {
    CriticalHeartRate(u32),
    HighHeartRate(u32),
    CriticalOxygenLevel(f64),
    LowOxygenLevel(f64),
    HighStressLevel(f64),
}

/// Condiciones médicas que se pueden predecir
#[derive(Debug, Clone)]
pub enum MedicalCondition {
    PossibleTachycardia,
    PossibleHypoxia,
    PossibleChronicStress,
    PossibleSleepApnea,
}

impl HealthAlert {
    pub fn severity(&self) -> &'static str {
        match self {
            Self::CriticalHeartRate(_) | Self::CriticalOxygenLevel(_) => "CRÍTICO",
            Self::HighHeartRate(_) | Self::LowOxygenLevel(_) => "ALTO",
            Self::HighStressLevel(_) => "MEDIO",
        }
    }

    pub fn message(&self) -> String {
        match self {
            Self::CriticalHeartRate(hr) => format!("Frecuencia cardíaca crítica: {} BPM", hr),
            Self::HighHeartRate(hr) => format!("Frecuencia cardíaca elevada: {} BPM", hr),
            Self::CriticalOxygenLevel(spo2) => format!("Nivel de oxígeno crítico: {:.1}%", spo2),
            Self::LowOxygenLevel(spo2) => format!("Nivel de oxígeno bajo: {:.1}%", spo2),
            Self::HighStressLevel(stress) => {
                format!("Nivel de estrés alto: {:.1}%", stress * 100.0)
            }
        }
    }
}
