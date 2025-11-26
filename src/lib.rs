pub mod config;
pub mod sensors;
pub mod navigation;
pub mod vision;
pub mod api;

// Re-export para fácil acceso
pub use config::Config;
pub use sensors::{process_sensor_data, filter_sensor_data};
