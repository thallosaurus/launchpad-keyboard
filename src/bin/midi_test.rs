use std::error::Error;

use lp_pro_gamecontroller::{mapping::Config, run};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_env()?;

    let config = Config::init("./Mapping.toml").await?;
    run(config).await?;
    Ok(())
}