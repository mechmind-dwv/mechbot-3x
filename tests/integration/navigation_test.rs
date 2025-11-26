use anyhow::Result;
use mechbot_3x::navigation::{a_star, NavigationController, SLAM};

#[cfg(test)]
mod navigation_integration_tests {
    use super::*;

    #[test]
    fn test_pathfinding_integration() -> Result<()> {
        let mut navigation = NavigationController::new();

        // Configurar posición inicial y target
        navigation.update_position(0.0, 0.0);
        navigation.set_target(5.0, 5.0);

        // Obstáculos simulados
        let obstacles = vec![(2.0, 2.0), (2.0, 3.0), (3.0, 2.0)];

        // Calcular ruta
        navigation.calculate_path(&obstacles)?;

        // Debería encontrar una ruta que evite obstáculos
        assert!(navigation.is_at_target(0.1)); // Dentro de la tolerancia

        Ok(())
    }

    #[test]
    fn test_complex_pathfinding() -> Result<()> {
        // Crear un laberinto de obstáculos
        let mut obstacles = Vec::new();

        // Pared vertical
        for y in 0..10 {
            obstacles.push((5.0, y as f64));
        }

        // Pared horizontal
        for x in 0..5 {
            obstacles.push((x as f64, 3.0));
        }

        let path = a_star((0.0, 0.0), (8.0, 8.0), &obstacles)?;

        // Debería encontrar un camino alrededor de las paredes
        assert!(!path.is_empty());
        assert_eq!(path[0], (0.0, 0.0));
        assert_eq!(path[path.len() - 1], (8.0, 8.0));

        // Verificar que el camino evita obstáculos
        for point in &path {
            assert!(!obstacles.contains(&(point.0.round(), point.1.round())));
        }

        Ok(())
    }

    #[test]
    fn test_slam_integration() -> Result<()> {
        let mut slam = SLAM::new(0.1, 100, 100); // 10cm resolución, 10x10m

        // Simular datos LIDAR desde diferentes posiciones
        let positions = vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0)];

        for (x, y) in positions {
            // Simular puntos LIDAR (obstáculos en un patrón circular)
            let mut points = Vec::new();
            for angle in 0..360 {
                let rad = angle as f64 * std::f64::consts::PI / 180.0;
                points.push((x + 2.0 * rad.cos(), y + 2.0 * rad.sin()));
            }

            slam.update_with_lidar((x, y), &points)?;
        }

        // Verificar que el mapa se actualizó
        let grid = slam.get_occupancy_grid();
        assert!(!grid.is_empty());

        // Debería haber celdas ocupadas (obstáculos)
        let occupied_cells: usize = grid
            .iter()
            .map(|row| row.iter().filter(|&&v| v > 0.5).count())
            .sum();

        assert!(occupied_cells > 0);

        Ok(())
    }

    #[test]
    fn test_navigation_loop() -> Result<()> {
        let mut navigation = NavigationController::new();

        // Simular un viaje paso a paso
        navigation.update_position(0.0, 0.0);
        navigation.set_target(3.0, 3.0);

        let obstacles = vec![(1.5, 1.5)]; // Obstáculo en el camino directo

        navigation.calculate_path(&obstacles)?;

        // Seguir la ruta punto por punto
        let mut current_pos = (0.0, 0.0);
        while let Some(waypoint) = navigation.get_next_waypoint() {
            current_pos = waypoint;
        }

        // Debería llegar al target
        assert!((current_pos.0 - 3.0).abs() < 0.1);
        assert!((current_pos.1 - 3.0).abs() < 0.1);

        Ok(())
    }
}

#[cfg(test)]
mod navigation_edge_cases {
    use super::*;

    #[test]
    fn test_no_path_available() {
        // Crear una situación sin ruta posible
        let obstacles = vec![
            (1.0, 0.0),
            (1.0, 1.0),
            (1.0, 2.0),
            (1.0, 3.0),
            (1.0, 4.0),
            (0.0, 1.0),
            (2.0, 1.0), // Cerrando el paso
        ];

        let result = a_star((0.0, 0.0), (3.0, 3.0), &obstacles);

        // Debería fallar cuando no hay ruta
        assert!(result.is_err());
    }

    #[test]
    fn test_same_start_and_goal() -> Result<()> {
        let path = a_star((5.0, 5.0), (5.0, 5.0), &[])?;

        // Ruta debería ser solo el punto de inicio
        assert_eq!(path, vec![(5.0, 5.0)]);

        Ok(())
    }

    #[test]
    fn test_large_environment() -> Result<()> {
        // Test con entorno grande
        let mut obstacles = Vec::new();

        // Crear obstáculos en patrón de grid
        for x in (0..50).step_by(5) {
            for y in (0..50).step_by(5) {
                if x != 0 || y != 0 {
                    // No obstaculizar el inicio
                    obstacles.push((x as f64, y as f64));
                }
            }
        }

        let path = a_star((0.0, 0.0), (49.0, 49.0), &obstacles)?;

        // Debería encontrar un camino a través del grid
        assert!(!path.is_empty());
        assert!(path.len() > 10); // Camino debería tener varios puntos

        Ok(())
    }
}
