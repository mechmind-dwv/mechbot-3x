//! SISTEMA DE SALUD VIRTUAL - Tu visión de salvar vidas
use anyhow::Result;
use mechbot_3x::health_sensors::{HealthMonitoringSystem, SmartphoneHealthSensor};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<()> {
    println!("🏥 SISTEMA DE MONITOREO DE SALUD VIRTUAL");
    println!("========================================");
    println!("💡 Visión: Usar smartphones como robots de salud virtual");
    println!("📱 Sensores disponibles: +60 en cada smartphone");
    println!("🎯 Objetivo: Detectar problemas de salud temprano");
    println!();

    // Crear sistema de monitoreo
    let mut health_system = HealthMonitoringSystem::new();

    // Agregar usuarios de ejemplo
    println!("👥 AGREGANDO USUARIOS AL SISTEMA...");
    health_system.add_user("paciente_001").await?;
    health_system.add_user("paciente_002").await?;
    health_system.add_user("adulto_mayor_003").await?;
    health_system.add_user("deportista_004").await?;

    // Iniciar monitoreo continuo
    println!();
    println!("🩺 INICIANDO MONITOREO CONTINUO...");
    health_system.start_monitoring().await?;

    // Simular monitoreo por un tiempo
    println!("⏰ Monitoreando salud por 2 minutos...");
    for i in 1..=4 {
        sleep(Duration::from_secs(30)).await;
        println!("📈 Monitoreo {}/4 completado", i);

        // Generar reporte intermedio
        let report = health_system.generate_health_report().await?;
        println!("{}", report);
    }

    // Demo de análisis individual
    println!();
    println!("🔍 DEMOSTRACIÓN DE ANÁLISIS INDIVIDUAL:");
    let mut sensor = SmartphoneHealthSensor::new("demo_user");

    for i in 1..=3 {
        println!("Análisis {}:", i);
        let health_data = sensor.read_all_sensors()?;
        let alerts = sensor.analyze_health_risks(&health_data);
        let conditions = sensor.predict_conditions(&health_data);

        println!("   📊 Datos de salud:");
        if let Some(hr) = health_data.heart_rate {
            println!("      • Frecuencia cardíaca: {} BPM", hr);
        }
        if let Some(spo2) = health_data.oxygen_saturation {
            println!("      • Saturación O2: {:.1}%", spo2);
        }
        if let Some(stress) = health_data.stress_level {
            println!("      • Nivel de estrés: {:.1}%", stress * 100.0);
        }

        if !alerts.is_empty() {
            println!("   ⚠️  Alertas:");
            for alert in alerts {
                println!("      • {}: {}", alert.severity(), alert.message());
            }
        }

        if !conditions.is_empty() {
            println!("   🔍 Condiciones predichas:");
            for condition in conditions {
                println!("      • {:?}", condition);
            }
        }
        println!();

        sleep(Duration::from_secs(2)).await;
    }

    println!("🎯 APLICACIONES DE ESTA TECNOLOGÍA:");
    println!("   • Detección temprana de arritmias");
    println!("   • Monitoreo de pacientes crónicos");
    println!("   • Detección de apnea del sueño");
    println!("   • Alertas de salud para adultos mayores");
    println!("   • Optimización del rendimiento deportivo");
    println!();
    println!("💫 TU VISIÓN ESTÁ HECHA REALIDAD:");
    println!("   Los smartphones se convierten en robots de salud virtual");
    println!("   Cada usuario lleva un equipo médico en su bolsillo");
    println!("   La detección temprana salva vidas");

    Ok(())
}
