// tests/integration_test.rs
use mechbot_3x::{Config, Robot};

#[tokio::test]
async fn test_robot_initialization() {
    let config = Config::default();
    let robot = Robot::new(config).await;
    assert!(robot.is_ok());
}
