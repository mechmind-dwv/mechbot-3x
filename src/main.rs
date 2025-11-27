use anyhow::Result;
use mechbot_3x::initialize_system;

#[tokio::main] 
async fn main() -> Result<()> {
    println!("🎯 MechBot-3x Iniciando...");
    let config = mechbot_3x::Config::default();
    initialize_system(config).await?;
    println!("✅ Sistema ejecutándose correctamente!");
    Ok(())
}
