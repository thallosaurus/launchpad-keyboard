use std::error::Error;
use log::Level;
use lp_pro_gamecontroller::{config::Config, main_event_loop, open_device_with_event_loop};

#[cfg(debug_assertions)]
const LOG_LEVEL: log::Level = Level::Trace;

#[cfg(not(debug_assertions))]
const LOG_LEVEL: log::Level = Level::Info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    simple_logger::init_with_level(LOG_LEVEL)?;

    let config = Config::init("./Mapping.toml").await?;
    open_device_with_event_loop(config.device, main_event_loop).await?;
    Ok(())
}