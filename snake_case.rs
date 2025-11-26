// Use snake_case para funciones y variables
pub fn calculate_distance(point_a: Point, point_b: Point) -> f64 {
    // ...
}

// Use PascalCase para tipos
pub struct RobotConfig {
    pub name: String,
    pub max_speed: f64,
}

// Documentación completa
/// Calcula la distancia entre dos puntos en el plano 2D.
///
/// # Arguments
/// * `point_a` - Primer punto
/// * `point_b` - Segundo punto
///
/// # Returns
/// Distancia euclidiana entre los puntos
///
/// # Examples
/// ```
/// let p1 = Point::new(0.0, 0.0);
/// let p2 = Point::new(3.0, 4.0);
/// assert_eq!(calculate_distance(p1, p2), 5.0);
/// ```
pub fn calculate_distance(point_a: Point, point_b: Point) -> f64 {
    point_a.distance_to(point_b)
}
