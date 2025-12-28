use std::{error::Error, io::{Write, stdin, stdout}};

use lp_pro_gamecontroller::{backend::event_loop, device::{connect_input, connect_output}, mapping::Config};
use simple_logger::SimpleLogger;
use tokio::sync::mpsc::channel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::init().await?;
    
    simple_logger::init_with_env().unwrap();

    let (tx, from_device) = channel(100);
    let _in_port = connect_input(config.get_input_name(), tx)?;
    let out_port = connect_output(config.get_output_name())?;

    event_loop(config, from_device, out_port).await.unwrap();
    Ok(())
}