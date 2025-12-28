use std::{collections::HashMap, sync::Arc};

use log::{debug, trace};
use midir::{MidiOutputConnection, SendError};
use tokio::sync::{Mutex, broadcast::{channel, error::RecvError}, mpsc::Receiver};
//use uinput::event::keyboard;

use crate::{mapping::{Config, MAPPING}, message::{Message, MidiMessage, Note}, virtual_input::{Actions, create_backend}};

pub async fn event_loop(config: Config, mut from_raw_device: Receiver<Message>, output_port: MidiOutputConnection) -> Result<(), RecvError> {
    let (tx, _rx) = channel(1);
    let mut in_rx = tx.subscribe();
    let mut out_rx = tx.subscribe();

    ctrlc::set_handler(move || { tx.send(()).expect("could not send ctrlc sig on channel"); })
        .expect("error setting ctrlc handler");

    let config = Arc::new(Mutex::new(config));

    //let input: Box<dyn VirtInput> = Box::new();
    let backend = Arc::new(Mutex::new(create_backend().expect("error while creating input backend")));
    
    let mut active: HashMap<Note, bool> = HashMap::new();
    {
        let lock = MAPPING.lock().unwrap();
        for k in lock.keys() {
            active.insert(*k, false);
        }
    }

    //let active = Arc::new(Mutex::new(active));
    let (active_tx, active_rx) = channel(100);

    //let input_map = actions_map.clone();
    let input_backend = backend.clone();
    //let active_in = active.clone();

    let _input_task = tokio::spawn(async move {
        //let map = input_map;
        //let active = active_in;
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

//                                        let mut alock = active.lock().await;
//                                        let a = alock.get_mut(&note).unwrap();
//                                        *a = true;

                                        //active.insert(note, true);
                                    }
                                },
                                MidiMessage::NoteOff(_ch, note) => {
                                    debug!("{:?}", note);
                                    let action: Option<Actions> = note.into();
                                    if let Some(action) = action {

                                        let mut lock = backend.lock().await;
                                        lock.process_off_action(action);
                                        drop(lock);

                                        //let mut alock = active.lock().await;
                                        
                                        //let a = alock.get_mut(&note).unwrap();
                                        //*a = false;
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
            //let msg = from_raw_device.recv().await;
            


        }
        // flush
    });

    //let out_config = config.clone();
    let active_out = active.clone();
    // displays the overlay and feedback
    let _output_task = tokio::spawn(async move {
        let active = active_out.clone();
        let output = Arc::new(std::sync::Mutex::new(output_port));

        let mut ac_rx = active_rx;

        draw_mapping(&output).await.unwrap();

        let _last_len = 0;
        loop {
            // draw diffs

            //let active = active.lock().await;
            //let mut lock = output.lock().expect("error acquiring output lock");

            /*for (n, a) in active.iter() {
                let msg= MidiMessage::NoteOn(0, *n, if *a {
                    120
                } else {
                    3
                });

                draw_active(msg, &output).await.unwrap();
                //lock.send(&msg);
            }
            drop(active);*/

            tokio::select! {
                Ok(msg) = ac_rx.recv() => {
                    match msg {
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
                        _ => {

                        }
                    }
                }
                _c = out_rx.recv() => {
                    debug!("closing output task");
                    break;
                }
            }
            
            //println!("Sent!");
        }

        // send all notes off
        send_all_off(&output).await
    });

    let _join = tokio::join!(_input_task, _output_task);
    Ok(())
}

async fn draw_active(message: MidiMessage, output: &Arc<std::sync::Mutex<MidiOutputConnection>>) -> Result<(), SendError> {
    let mut lock = output.lock().expect("error acquiring output lock");
    let msg: Vec<u8> = message.into();
    lock.send(&msg)?;
    Ok(())
}

async fn draw_mapping(output: &Arc<std::sync::Mutex<MidiOutputConnection>>) -> Result<(), SendError> {
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

async fn send_all_off(output: &Arc<std::sync::Mutex<MidiOutputConnection>>) -> Result<(), SendError> {
        let mut lock = output.lock().expect("error acquiring output lock");
    for i in 0..=127 {
        let index = i as u8;
        let note: Vec<u8> = MidiMessage::NoteOff(0, index.into()).into();
        lock.send(&note)?;
    }
    Ok(())
}