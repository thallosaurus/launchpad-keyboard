use std::{error::Error, io::{Write, stdin, stdout}};

use lp_pro_gamecontroller::{backend::event_loop, device::{connect_input, connect_output}};
use simple_logger::SimpleLogger;
use tokio::sync::mpsc::channel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new().init().unwrap();
    //let device = connect().unwrap();
    let (tx, from_device) = channel(100);

    let in_port = connect_input(tx)?;
    let out_port = connect_output()?;

    event_loop(from_device, out_port).await.unwrap();
    Ok(())
}