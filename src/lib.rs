#![doc = concat!(
   include_str!("../README.md")
)]

use crate::midi::input::{daw_mode_task, input_task};
use crate::midi::output::{OutputDeviceNameRetrieve, start_overlay_task};
use crate::{
    config::Config,
    midi::{device, message::Message},
    virtual_input::create_backend,
};
use midir::MidiOutputConnection;
use std::{
    error::Error,
    sync::{Arc, mpsc::RecvError},
};
use tokio::sync::{Mutex, broadcast, mpsc};

/// Midi related stuff
pub(crate) mod midi;

/// Mapping related stuff
pub mod config;

/// Stuff that emulates the keyboard
pub(crate) mod virtual_input;

pub trait DeviceNameRetrieve {
    fn get_input_name(&self) -> Option<String>;
    fn get_output_name(&self) -> Option<String>;
}

/// Run the specified event loop using the given config
pub async fn open_device_with_event_loop<F, Fut, C>(
    config: C,
    event_loop: F,
) -> Result<(), Box<dyn Error>>
where
    F: Fn(C, mpsc::Receiver<Message>, MidiOutputConnection) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<(), RecvError>> + Send,
    C: DeviceNameRetrieve,
{
    let (tx, from_device) = mpsc::channel(100);
    let _in_port = device::connect_input(config.get_input_name(), tx)?;
    let out_port = device::connect_output(config.get_output_name())?;

    event_loop(config, from_device, out_port).await?;
    Ok(())
}

/// The Event Loop that processes and sends events to their destinations
pub async fn main_event_loop<C>(
    config: C,
    from_raw_device: mpsc::Receiver<Message>,
    output_port: MidiOutputConnection,
) -> Result<(), RecvError>
where
    C: OutputDeviceNameRetrieve + Send + Sync + Clone + 'static,
{
    // cancellation signal that signals our tasks we are done
    let (cancellation, _rx) = broadcast::channel(1);
    let in_rx = cancellation.subscribe();
    let out_rx = cancellation.subscribe();

    ctrlc::set_handler(move || {
        cancellation
            .send(())
            .expect("could not send ctrlc sig on channel");
    })
    .expect("error setting ctrlc handler");

    let backend = Arc::new(Mutex::new(
        create_backend().expect("error while creating input backend"),
    ));

    // feedback channel
    let (active_tx, active_rx) = broadcast::channel(100);

    let input_task = tokio::spawn(input_task(from_raw_device, backend, active_tx, in_rx));
    let output_task = tokio::spawn(start_overlay_task(
        config.clone(),
        active_rx,
        output_port,
        out_rx,
    ));

    //let daw_mode_in = tokio::spawn(daw_mode_task(from_raw_device, backend, internal_broadcast, cancellation))

    let join = tokio::join!(input_task, output_task);

    Ok(())
}