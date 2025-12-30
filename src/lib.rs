//! # Launchpad Pro Userspace Keyboard Driver
//! 
//! This library allows you to map MIDI Devices to Keyboard Inputs making them effective computer keyboards.

use std::{error::Error, sync::{Arc, mpsc::RecvError}};
use midir::MidiOutputConnection;
use tokio::sync::{Mutex, broadcast, mpsc};
use crate::{mapping::Config, midi::{device, message::Message}, virtual_input::create_backend};
use crate::midi::output::start_overlay_task;
use crate::midi::input::start_input_task;

/// Midi related stuff
mod midi;

/// Mapping related stuff
pub mod mapping;

/// Stuff that emulates the keyboard
mod virtual_input;



/// Run the default event loop using the given config
pub async fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let (tx, from_device) = mpsc::channel(100);
    let _in_port = device::connect_input(config.get_input_name(), tx)?;
    let out_port = device::connect_output(config.get_output_name())?;

    event_loop(config, from_device, out_port).await?;
    Ok(())
}

/// The Event Loop that processes and sends events to their destinations
pub async fn event_loop(
    config: Config,
    from_raw_device: mpsc::Receiver<Message>,
    output_port: MidiOutputConnection,
) -> Result<(), RecvError> {
    // cancellation signal that signals our tasks we are done
    let (tx, _rx) = broadcast::channel(1);
    let in_rx = tx.subscribe();
    let out_rx = tx.subscribe();

    ctrlc::set_handler(move || {
        tx.send(()).expect("could not send ctrlc sig on channel");
    })
    .expect("error setting ctrlc handler");

    let backend = Arc::new(Mutex::new(
        create_backend().expect("error while creating input backend"),
    ));

    // feedback channel
    let (active_tx, active_rx) = broadcast::channel(100);

    let input_task = tokio::spawn(start_input_task(config.clone(), from_raw_device, backend, active_tx, in_rx));
    let output_task = tokio::spawn(start_overlay_task(config.clone(), active_rx, output_port, out_rx));
    let join = tokio::join!(
        input_task,
        output_task
    );

    Ok(())
}
