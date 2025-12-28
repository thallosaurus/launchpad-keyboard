use std::{collections::HashMap, sync::Arc};

use log::{debug, trace};
use midir::{MidiOutputConnection, SendError};
use tokio::sync::{
    Mutex,
    broadcast::{channel, error::RecvError},
    mpsc::Receiver,
};

use crate::{
    mapping::{Config, MAPPING},
    message::{Message, MidiMessage, Note},
    virtual_input::{Actions, create_backend},
};

pub async fn event_loop(
    config: Config,
    mut from_raw_device: Receiver<Message>,
    output_port: MidiOutputConnection,
) -> Result<(), RecvError> {
    let (tx, _rx) = channel(1);
    let mut in_rx = tx.subscribe();
    let mut out_rx = tx.subscribe();

    ctrlc::set_handler(move || {
        tx.send(()).expect("could not send ctrlc sig on channel");
    })
    .expect("error setting ctrlc handler");

    let backend = Arc::new(Mutex::new(
        create_backend().expect("error while creating input backend"),
    ));

    let (active_tx, active_rx) = channel(100);

    let input_backend = backend.clone();

    let _input_task = tokio::spawn(async move {
        let ac_tx = active_tx;

        let backend = input_backend;

        loop {
            tokio::select! {
                msg = from_raw_device.recv() => {
                    let _msg_clone = msg.clone();
                    match msg {
                        Some(msg) => {
                            match msg.1 {
                                MidiMessage::NoteOn(_ch, note, _vel) => {
                                    debug!("{:?}", note);
                                    let action: Option<Actions> = note.into();
                                    if let Some(action) = action {

                                        let mut lock = backend.lock().await;
                                        lock.process_on_action(action);
                                        drop(lock);

                                        // send to overlay
                                        ac_tx.send(msg.1).unwrap();
                                    }
                                },
                                MidiMessage::NoteOff(_ch, note) => {
                                    debug!("{:?}", note);
                                    let action: Option<Actions> = note.into();
                                    if let Some(action) = action {

                                        let mut lock = backend.lock().await;
                                        lock.process_off_action(action);
                                        drop(lock);

                                        ac_tx.send(msg.1).unwrap();
                                    }
                                },
                                MidiMessage::AfterTouch(_ch, note, _vel) => {
                                    debug!("{:?}", note);
                                    let action: Option<Actions> = note.into();

                                    if let Some(_action) = action {
                                        // todo
                                    }
                                },
                                MidiMessage::Unknown => {
                                    // do nothing
                                    continue
                                },
                            }
                        },
                        None => {
                            // channel has been closed
                            break;
                        },
                    }
                }
                _c = in_rx.recv() => {
                    debug!("closing input task");
                    break;
                }
            }
        }
        // flush
    });

    // displays the overlay and feedback
    let _output_task = tokio::spawn(async move {
        //let active = active_out.clone();
        let output = Arc::new(std::sync::Mutex::new(output_port));

        let mut ac_rx = active_rx;

        draw_mapping(&output).await.unwrap();

        let _last_len = 0;
        loop {
            tokio::select! {
                Ok(msg) = ac_rx.recv() => {

                    draw_active(msg, &output).await?;
                }
                _c = out_rx.recv() => {
                    debug!("closing output task");
                    break;
                }
            }
        }

        // send all notes off
        send_all_off(&output).await
    });

    let _join = tokio::join!(_input_task, _output_task);
    Ok(())
}

async fn draw_active(
    message: MidiMessage,
    output: &Arc<std::sync::Mutex<MidiOutputConnection>>,
) -> Result<(), SendError> {
    match message {
        MidiMessage::NoteOn(ch, note, vel) => {
            let new_msg: Vec<u8> = MidiMessage::NoteOn(0, note, 120).into();
            let mut lock = output.lock().expect("error acquiring output lock");
            lock.send(&new_msg).unwrap();
            trace!("{:?}", new_msg);
        }

        MidiMessage::NoteOff(ch, note) => {
            let new_msg: Vec<u8> = MidiMessage::NoteOn(0, note, 11).into();
            let mut lock = output.lock().expect("error acquiring output lock");
            lock.send(&new_msg).unwrap();
            trace!("{:?}", new_msg);
        }
        _ => {}
    }

    Ok(())
}

async fn draw_mapping(
    output: &Arc<std::sync::Mutex<MidiOutputConnection>>,
) -> Result<(), SendError> {
    let mapping = MAPPING.lock().unwrap();

    let mut lock = output.lock().expect("error acquiring output lock");

    for m in mapping.keys() {
        let msg: Vec<u8> = MidiMessage::NoteOn(0, *m, 11).into();
        debug!("{:?}", msg);
        lock.send(&msg)?;
    }
    drop(lock);
    Ok(())
}

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
