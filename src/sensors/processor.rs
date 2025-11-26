use log::{debug, info, warn, error};
use anyhow::Result;

pub fn process_sensor_data(sensor_readings: &[f64]) -> Result<()> {
    debug!("Procesando {} lecturas de sensor", sensor_readings.len());
    
    if sensor_readings.is_empty() {
        warn!("No hay datos de sensor para procesar");
        return Ok(());
    }
    
    // Procesar datos
    let average: f64 = sensor_readings.iter().sum::<f64>() / sensor_readings.len() as f64;
    
    if average > 100.0 {
        error!("Valor de sensor fuera de rango: {}", average);
        return Err(anyhow::anyhow!("Sensor overload"));
    }
    
    info!("Procesamiento completado - promedio: {:.2}", average);
    Ok(())
}

// Función adicional para filtrar datos anómalos
pub fn filter_sensor_data(readings: &[f64], threshold: f64) -> Vec<f64> {
    readings.iter()
        .filter(|&&x| x <= threshold)
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_sensor_data() {
        let readings = vec![10.0, 20.0, 30.0];
        assert!(process_sensor_data(&readings).is_ok());
    }

    #[test]
    fn test_empty_sensor_data() {
        let readings = vec![];
        assert!(process_sensor_data(&readings).is_ok());
    }
}
