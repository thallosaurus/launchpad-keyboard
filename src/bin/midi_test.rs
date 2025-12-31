use std::error::Error;
use log::Level;
use lp_pro_gamecontroller::{config::Config, integration_event_loop, main_event_loop, open_device_pair_with_event_loop};
use tokio::join;

#[cfg(debug_assertions)]
const LOG_LEVEL: log::Level = Level::Debug;

#[cfg(not(debug_assertions))]
const LOG_LEVEL: log::Level = Level::Info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    simple_logger::init_with_level(LOG_LEVEL)?;

    let config = Config::init("./Mapping.toml").await?;
    let j = join!(
        open_device_pair_with_event_loop(config.device, main_event_loop),
        //open_device_pair_with_event_loop(config.integration, integration_event_loop)
    );
    Ok(())
}