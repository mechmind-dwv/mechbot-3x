//! Ejemplo de control remoto de MechBot-3x
//!
//! Este ejemplo demuestra cómo controlar el robot remotamente mediante:
//! - API REST para comandos
//! - WebSocket para telemetría en tiempo real
//! - Interfaz de control manual y automático

use anyhow::Result;
use mechbot_3x::{
    api::{ApiServer, RestApi},
    initialize_system,
    sensors::Lidar,
    Config,
};
use std::collections::VecDeque;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<()> {
    println!("🌐 Iniciando Sistema de Control Remoto...");

    let config = Config {
        api: mechbot_3x::config::ApiConfig {
            rest_port: Some(8080),
            websocket_port: Some(8081),
            enable_cors: Some(true),
            api_key_required: Some(false), // Para testing
        },
        ..Default::default()
    };

    let (mut lidar, mut camera, mut imu, mut navigation, mut vision, mut api_server) =
        initialize_system(config).await?;

    println!("✅ Sistema de control remoto inicializado");
    println!("📡 Endpoints disponibles:");
    println!("   - REST API: http://localhost:8080");
    println!("   - WebSocket: ws://localhost:8081");
    println!("   - Comandos: POST /api/v1/move");
    println!("   - Estado: GET /api/v1/status");
    println!("   - Sensores: GET /api/v1/sensors");

    // Simular cliente conectándose
    println!("\n🔗 Clientes simulados conectados:");
    println!("   📱 App móvil - Control táctil");
    println!("   🖥️  Dashboard web - Monitoreo en tiempo real");
    println!("   🎮 Control remoto físico - Comandos manuales");

    // Buffer para comandos recibidos
    let mut command_queue: VecDeque<RobotCommand> = VecDeque::new();

    // Simular recepción de comandos
    simulate_remote_commands(&mut command_queue).await;

    println!("\n🎮 Iniciando procesamiento de comandos remotos...");

    let mut iteration = 0;
    while api_server.is_running() && iteration < 200 {
        // Limitar para el ejemplo
        iteration += 1;

        // ===== PROCESAR COMANDOS RECIBIDOS =====
        if let Some(command) = command_queue.pop_front() {
            println!("📨 Comando recibido: {:?}", command);

            match command {
                RobotCommand::MoveTo(x, y) => {
                    println!("   🎯 Navegando a posición: ({}, {})", x, y);
                    navigation.set_target(x, y);
                }
                RobotCommand::Stop => {
                    println!("   🛑 Deteniendo robot");
                    // En implementación real, detendría motores
                }
                RobotCommand::SetSpeed(speed) => {
                    println!("   🚀 Configurando velocidad: {}", speed);
                }
                RobotCommand::EmergencyStop => {
                    println!("   🚨 PARADA DE EMERGENCIA!");
                    // Detener inmediatamente todos los sistemas
                    break;
                }
            }
        }

        // ===== ACTUALIZAR TELEMETRÍA =====
        let sensor_data = tokio::try_join!(
            async { lidar.scan_environment() },
            async { camera.capture_frame() },
            async { imu.get_orientation() },
        )?;

        let (lidar_points, camera_frame, imu_orientation) = sensor_data;
        let obstacles = lidar.get_obstacles(&lidar_points, 2.0);

        // Procesar visión si hay frame
        let detections =
            vision.process_frame(&camera_frame.data, camera_frame.width, camera_frame.height)?;

        // Enviar telemetría en tiempo real
        let telemetry = format!(
            r#"{{
                "timestamp": {},
                "position": {:?},
                "obstacles": {},
                "detections": {},
                "orientation": {:?},
                "battery": {},
                "status": "operational"
            }}"#,
            chrono::Utc::now().timestamp(),
            navigation.current_position,
            obstacles.len(),
            detections.len(),
            imu_orientation,
            85 - (iteration / 10) // Simular descarga de batería
        );

        api_server.send_telemetry(&telemetry)?;

        // ===== EJECUTAR NAVEGACIÓN AUTÓNOMA =====
        if !navigation.is_at_target(0.1) {
            if let Some(next_waypoint) = navigation.get_next_waypoint() {
                navigation.update_position(next_waypoint.0, next_waypoint.1);

                if iteration % 10 == 0 {
                    println!(
                        "   🧭 Navegando automáticamente a: ({:.2}, {:.2})",
                        next_waypoint.0, next_waypoint.1
                    );
                }
            }
        }

        // Simular recepción ocasional de nuevos comandos
        if iteration % 15 == 0 {
            simulate_remote_commands(&mut command_queue).await;
        }

        sleep(Duration::from_millis(100)).await;

        if iteration % 50 == 0 {
            println!(
                "📊 Iteración {} - Comandos en cola: {}",
                iteration,
                command_queue.len()
            );
        }
    }

    println!("\n📊 Resumen de sesión de control remoto:");
    println!("   - Iteraciones completadas: {}", iteration);
    println!("   - Comandos procesados: {}", 200 - command_queue.len());
    println!("   - Posición final: {:?}", navigation.current_position);

    api_server.stop();
    println!("🌐 Sesión de control remoto finalizada");

    Ok(())
}

/// Comandos que pueden recibirse remotamente
#[derive(Debug, Clone)]
enum RobotCommand {
    MoveTo(f64, f64),
    Stop,
    SetSpeed(f64),
    EmergencyStop,
}

/// Simular recepción de comandos desde clientes remotos
async fn simulate_remote_commands(queue: &mut VecDeque<RobotCommand>) {
    // Simular diferentes tipos de clientes enviando comandos

    // App móvil - comandos táctiles
    queue.push_back(RobotCommand::MoveTo(5.0, 3.0));

    // Dashboard web - monitoreo y control fino
    queue.push_back(RobotCommand::SetSpeed(0.8));

    // Secuencia automática
    for i in 0..3 {
        queue.push_back(RobotCommand::MoveTo(i as f64 * 2.0, i as f64 * 1.5));
    }
}

/// Ejemplo de interfaz de control manual
pub async fn manual_control_example() -> Result<()> {
    println!("🎮 Iniciando modo control manual...");

    let config = Config::default();
    let (mut lidar, _, _, mut navigation, _, mut api_server) = initialize_system(config).await?;

    println!("✅ Control manual activado");
    println!("📋 Comandos disponibles:");
    println!("   W - Avanzar");
    println!("   S - Retroceder");
    println!("   A - Izquierda");
    println!("   D - Derecha");
    println!("   SPACE - Detener");
    println!("   Q - Salir");

    // En una implementación real, aquí habría una interfaz de usuario
    // Para este ejemplo, simulamos comandos

    let manual_commands = vec!['W', 'W', 'D', 'W', 'A', 'S', ' '];

    for (i, &cmd) in manual_commands.iter().enumerate() {
        println!("\n⌨️  Comando {}/{}: {}", i + 1, manual_commands.len(), cmd);

        match cmd {
            'W' => {
                navigation.update_position(
                    navigation.current_position.0,
                    navigation.current_position.1 + 1.0,
                );
                println!("   ↑ Avanzando...");
            }
            'S' => {
                navigation.update_position(
                    navigation.current_position.0,
                    navigation.current_position.1 - 1.0,
                );
                println!("   ↓ Retrocediendo...");
            }
            'A' => {
                navigation.update_position(
                    navigation.current_position.0 - 1.0,
                    navigation.current_position.1,
                );
                println!("   ← Izquierda...");
            }
            'D' => {
                navigation.update_position(
                    navigation.current_position.0 + 1.0,
                    navigation.current_position.1,
                );
                println!("   → Derecha...");
            }
            ' ' => {
                println!("   ⏹️  Detenido");
            }
            _ => {}
        }

        // Verificar obstáculos incluso en modo manual
        if let Ok(points) = lidar.scan_environment() {
            let obstacles = lidar.get_obstacles(&points, 0.8);
            if !obstacles.is_empty() {
                println!("   ⚠️  CUIDADO! Obstáculo cercano!");
            }
        }

        // Enviar telemetría
        api_server.send_telemetry(&format!(
            r#"{{"manual_control": true, "command": "{}", "position": {:?}}}"#,
            cmd, navigation.current_position
        ))?;

        sleep(Duration::from_millis(500)).await;
    }

    println!("\n✅ Sesión de control manual completada");
    api_server.stop();

    Ok(())
}
