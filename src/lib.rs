//! # Launchpad Pro Userspace Keyboard Driver
//! 
//! This library allows you to map MIDI Devices to Keyboard Inputs making them effective computer keyboards.

use std::error::Error;
use tokio::sync::mpsc;
use crate::mapping::Config;

/// Main Event Loop
mod backend;

/// MIDI Message related stuff
mod message;

/// MIDI Device Stuff
mod device;

/// Mapping related stuff
pub mod mapping;

/// Stuff that emulates the keyboard
mod virtual_input;

mod note;

/// Run the default event loop using the given config
pub async fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let (tx, from_device) = mpsc::channel(100);
    let _in_port = device::connect_input(config.get_input_name(), tx)?;
    let out_port = device::connect_output(config.get_output_name())?;

    backend::event_loop(config, from_device, out_port).await?;
    Ok(())
}