use std::sync::Arc;

use log::{debug, info, trace};
use tokio::{
    sync::{Mutex, broadcast, mpsc},
    task::JoinHandle,
};

use crate::{
    DeviceNameRetrieve,
    config::{Action, Config},
    midi::message::{Message, MidiMessage},
    virtual_input::InputBackend,
};

pub enum InputTaskError {}

pub async fn daw_mode_task(
    mut from_raw_device: mpsc::Receiver<Message>,
    backend: Arc<Mutex<Box<dyn InputBackend>>>,
    internal_broadcast: broadcast::Sender<MidiMessage>,
    mut cancellation: broadcast::Receiver<()>,
) -> Result<(), InputTaskError> {
    loop {
        tokio::select! {
            msg = from_raw_device.recv() => {
                info!("{:?}", msg);
            }
            _c = cancellation.recv() => {
                debug!("closing input task");
                break;
            }
        }
    }
    Ok(())
}

pub async fn input_task(
    mut from_raw_device: mpsc::Receiver<Message>,
    backend: Arc<Mutex<Box<dyn InputBackend>>>,
    internal_broadcast: broadcast::Sender<MidiMessage>,
    mut cancellation: broadcast::Receiver<()>,
) -> Result<(), InputTaskError> {
    //tokio::spawn(async move {
    loop {
        tokio::select! {
            msg = from_raw_device.recv() => {
                let _msg_clone = msg.clone();
                match msg {
                    Some(msg) => {
                        match msg.1 {
                            MidiMessage::NoteOn(_ch, note, _vel) => {
                                trace!("{:?}", note);
                                let action: Option<Action> = note.into();
                                if let Some(action) = action {

                                    let mut lock = backend.lock().await;
                                    lock.process_on_action(action);
                                    drop(lock);

                                    // send to overlay
                                    internal_broadcast.send(msg.1).unwrap();
                                }
                            },
                            MidiMessage::NoteOff(_ch, note) => {
                                trace!("{:?}", note);
                                let action: Option<Action> = note.into();
                                if let Some(action) = action {

                                    let mut lock = backend.lock().await;
                                    lock.process_off_action(action);
                                    drop(lock);

                                    internal_broadcast.send(msg.1).unwrap();
                                }
                            },
                            MidiMessage::AfterTouch(_ch, note, _vel) => {
                                trace!("{:?}", note);
                                let action: Option<Action> = note.into();

                                if let Some(_action) = action {
                                    // todo
                                }
                            },
                            MidiMessage::Unknown => {
                                // do nothing
                                debug!("Unknown: {:?}", msg.1);
                                continue
                            },
                            MidiMessage::Clock => {
                                trace!("Clock");
                                continue
                            }
                        }
                    },
                    None => {
                        // channel has been closed
                        break;
                    },
                }
            }
            _c = cancellation.recv() => {
                debug!("closing input task");
                break;
            }
        }
    }
    Ok(())
    //})
}
