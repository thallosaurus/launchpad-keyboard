use std::sync::Arc;

use log::{debug, info, trace};
use midir::{MidiOutputConnection, SendError};
use tokio::sync::broadcast;

use crate::{
    DeviceNameRetrieve,
    config::Config,
    midi::{message::MidiMessage, note::MAPPING},
};

type OutputTaskReturn = Result<(), SendError>;

const COLOR_PAD_ON: u8 = 120;
const COLOR_PAD_OFF: u8 = 11;

pub trait OutputDeviceNameRetrieve: DeviceNameRetrieve {
    fn get_light_status(&self) -> bool;
}

pub async fn start_overlay_task<C>(
    config: C,
    mut receiver: broadcast::Receiver<MidiMessage>,
    output_port: MidiOutputConnection,
    mut cancellation: broadcast::Receiver<()>,
) -> OutputTaskReturn
where
    C: OutputDeviceNameRetrieve,
{
    let output_port = Arc::new(std::sync::Mutex::new(output_port));
    if config.get_light_status() {
        draw_mapping(&output_port).await?;

        let _last_len = 0;
        loop {
            tokio::select! {
                Ok(msg) = receiver.recv() => {

                    draw_active(msg, &output_port).await?;
                }
                _c = cancellation.recv() => {
                    debug!("closing output task");
                    break;
                }
            }
        }

        // send all notes off
        send_all_off(&output_port).await?;
        Ok(())
    } else {
        info!("Overlay is disabled!");
        Ok(())
    }
}

/// Draws the velocities on the hardware
async fn draw_active(
    message: MidiMessage,
    output: &Arc<std::sync::Mutex<MidiOutputConnection>>,
) -> Result<(), SendError> {
    match message {
        MidiMessage::NoteOn(ch, note, vel) => {
            let new_msg: Vec<u8> = MidiMessage::NoteOn(0, note, COLOR_PAD_ON).into();
            let mut lock = output.lock().expect("error acquiring output lock");
            lock.send(&new_msg).unwrap();
            trace!("{:?}", new_msg);
        }

        MidiMessage::NoteOff(ch, note) => {
            let new_msg: Vec<u8> = MidiMessage::NoteOn(0, note, COLOR_PAD_OFF).into();
            let mut lock = output.lock().expect("error acquiring output lock");
            lock.send(&new_msg).unwrap();
            trace!("{:?}", new_msg);
        }
        _ => {}
    }

    Ok(())
}

/// Initially draw the mapping on the device
async fn draw_mapping(
    output: &Arc<std::sync::Mutex<MidiOutputConnection>>,
) -> Result<(), SendError> {
    let mapping = MAPPING.lock().unwrap();

    let mut lock = output.lock().expect("error acquiring output lock");

    debug!("Sending Overlay: {:?}", mapping.keys());
    for m in mapping.keys() {
        let msg: Vec<u8> = MidiMessage::NoteOn(0, *m, COLOR_PAD_OFF).into();
        lock.send(&msg)?;
    }
    drop(lock);
    Ok(())
}

/// Send MidiOff to all notes
async fn send_all_off(
    output: &Arc<std::sync::Mutex<MidiOutputConnection>>,
) -> Result<(), SendError> {
    let mut lock = output.lock().expect("error acquiring output lock");
    for i in 0..=127 {
        let index = i as u8;
        let note: Vec<u8> = MidiMessage::NoteOff(0, index.into()).into();
        lock.send(&note)?;
    }
    Ok(())
}
