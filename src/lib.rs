pub mod api;
pub mod config; 
pub mod control;
pub mod navigation;
pub mod sensors;
pub mod vision;

pub use config::Config;

pub async fn initialize_system(config: Config) -> anyhow::Result<()> {
    println!("🚀 Sistema inicializado con config: {}", config.robot.name);
    Ok(())
}
