//! Sistema de monitoreo continuo de salud
use super::{HealthData, HealthAlert, MedicalCondition, SmartphoneHealthSensor};
use anyhow::Result;
use tokio::time::{interval, Duration};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Sistema de monitoreo continuo que analiza datos en tiempo real
pub struct HealthMonitoringSystem {
    sensors: Arc<RwLock<Vec<SmartphoneHealthSensor>>>,
    alert_history: Arc<RwLock<Vec<HealthAlert>>>,
    is_monitoring: bool,
}

impl HealthMonitoringSystem {
    pub fn new() -> Self {
        Self {
            sensors: Arc::new(RwLock::new(Vec::new())),
            alert_history: Arc::new(RwLock::new(Vec::new())),
            is_monitoring: false,
        }
    }

    /// Agrega un usuario al sistema de monitoreo
    pub async fn add_user(&self, user_id: &str) -> Result<()> {
        let mut sensors = self.sensors.write().await;
        sensors.push(SmartphoneHealthSensor::new(user_id));
        println!("👤 Usuario {} agregado al monitoreo de salud", user_id);
        Ok(())
    }

    /// Inicia el monitoreo continuo
    pub async fn start_monitoring(&mut self) -> Result<()> {
        self.is_monitoring = true;
        println!("🩺 INICIANDO MONITOREO CONTINUO DE SALUD");
        
        let sensors = self.sensors.clone();
        let alert_history = self.alert_history.clone();
        
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(30)); // Monitoreo cada 30 segundos
            
            while true { // En una app real, esto estaría controlado
                interval.tick().await;
                
                let sensors_guard = sensors.read().await;
                for sensor in sensors_guard.iter() {
                    // En una implementación real, aquí se leerían sensores reales
                    // Por ahora simulamos la lectura
                    match Self::simulate_sensor_reading(sensor).await {
                        Ok((health_data, alerts, conditions)) => {
                            if !alerts.is_empty() {
                                println!("⚠️  ALERTAS DE SALUD DETECTADAS:");
                                for alert in &alerts {
                                    println!("   • {}: {}", alert.severity(), alert.message());
                                }
                                
                                // Enviar alertas (en app real: notificaciones push, SMS, etc.)
                                let mut history = alert_history.write().await;
                                history.extend(alerts);
                            }
                            
                            if !conditions.is_empty() {
                                println!("🔍 CONDICIONES PREDICHAS:");
                                for condition in conditions {
                                    println!("   • {:?}", condition);
                                }
                            }
                        }
                        Err(e) => eprintln!("❌ Error leyendo sensores: {}", e),
                    }
                }
            }
        });
        
        Ok(())
    }

    /// Simula la lectura de sensores (en app real se conectaría a APIs de salud)
    async fn simulate_sensor_reading(
        sensor: &SmartphoneHealthSensor
    ) -> Result<(HealthData, Vec<HealthAlert>, Vec<MedicalCondition>)> {
        // En una implementación real, esto se conectaría a:
        // - HealthKit (iOS)
        // - Google Fit (Android)  
        // - Samsung Health
        // - APIs de wearables (Apple Watch, Fitbit, etc.)
        
        let mut simulated_sensor = SmartphoneHealthSensor::new(&sensor.user_id);
        let health_data = simulated_sensor.read_all_sensors()?;
        let alerts = simulated_sensor.analyze_health_risks(&health_data);
        let conditions = simulated_sensor.predict_conditions(&health_data);
        
        Ok((health_data, alerts, conditions))
    }

    /// Obtiene el historial de alertas
    pub async fn get_alert_history(&self) -> Vec<HealthAlert> {
        let history = self.alert_history.read().await;
        history.clone()
    }

    /// Genera un reporte de salud consolidado
    pub async fn generate_health_report(&self) -> Result<String> {
        let sensors = self.sensors.read().await;
        let alerts = self.alert_history.read().await;
        
        let mut report = String::new();
        report.push_str("📊 REPORTE DE SALUD CONSOLIDADO\n");
        report.push_str("================================\n");
        report.push_str(&format!("Usuarios monitoreados: {}\n", sensors.len()));
        report.push_str(&format!("Alertas registradas: {}\n", alerts.len()));
        
        if !alerts.is_empty() {
            report.push_str("\n📈 ALERTAS RECIENTES:\n");
            for alert in alerts.iter().rev().take(5) { // Últimas 5 alertas
                report.push_str(&format!("• {}: {}\n", alert.severity(), alert.message()));
            }
        }
        
        Ok(report)
    }
}

impl Default for HealthMonitoringSystem {
    fn default() -> Self {
        Self::new()
    }
}
