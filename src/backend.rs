use std::{collections::HashMap, sync::Arc};

use log::debug;
use midir::MidiOutputConnection;
use tokio::sync::{Mutex, broadcast::{channel, error::RecvError}, mpsc::Receiver};
//use uinput::event::keyboard;

use crate::{mapping::Config, message::{Message, MidiMessage, MidiVelocity}, virtual_input::{Actions, InputBackend, create_backend}};

type ActiveActions = HashMap<Actions, MidiVelocity>;

pub async fn event_loop(config: Config, mut from_raw_device: Receiver<Message>, _output_port: MidiOutputConnection) -> Result<(), RecvError> {
    let (tx, _rx) = channel(1);
    let mut in_rx = tx.subscribe();
    let mut out_rx = tx.subscribe();

    ctrlc::set_handler(move || { tx.send(()).expect("could not send ctrlc sig on channel"); })
        .expect("error setting ctrlc handler");

    /*let mut device = uinput::default().unwrap()
        .name("launchpad-keyboard").unwrap()
        .event(uinput::event::Keyboard::All).unwrap()
        .create().unwrap();*/

    //let actions_map = Arc::new(Mutex::new(ActiveActions::new()));
    let config = Arc::new(Mutex::new(config));

    //let input: Box<dyn VirtInput> = Box::new();
    let backend = Arc::new(Mutex::new(create_backend().expect("error while creating input backend")));

    //let input_map = actions_map.clone();
    let input_backend = backend.clone();
    let _input_task = tokio::spawn(async move {
        //let map = input_map;
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
                                        //let mut map = map.lock().await;

                                        //if !map.contains_key(&action) {
                                        //    map.insert(action, vel);
                                        //}
                                        //drop(map);

                                        let mut lock = backend.lock().await;
                                        lock.process_on_action(action);
                                        drop(lock);

                                        /*let key: keyboard::Key = action.into();
                                        device.press(&key).unwrap();
                                        device.synchronize().unwrap();*/
                                    }
                                },
                                MidiMessage::NoteOff(_ch, note) => {
                                    debug!("{:?}", note);
                                    let action: Option<Actions> = note.into();
                                    if let Some(action) = action {

                                        let mut lock = backend.lock().await;
                                        lock.process_off_action(action);
                                        drop(lock);
                                        
                                        /*let mut map = map.lock().await;
                                        if map.contains_key(&action) {
                                            map.remove(&action);
                                        }
                                        drop(map);*/
                                        
                                        /*
                                        let key: keyboard::Key = action.into();
                                        device.release(&key).unwrap();
                                        device.synchronize().unwrap();*/
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

    let out_config = config.clone();
    // displays the overlay and feedback
    let _output_task = tokio::spawn(async move {
        //let msg = [144, 36, 125];
        //output_port.send(&msg).unwrap();

        let _config = out_config.lock().await;

        let _last_len = 0;
        loop {
            tokio::select! {
                _c = out_rx.recv() => {
                    debug!("closing output task");
                    break;
                }
            }
            
            //println!("Sent!");
        }

        // send all notes off
    });

    let _join = tokio::join!(_input_task, _output_task);
    Ok(())
}

#[derive(Clone, Copy, Debug)]
enum BroadcastClose {
    Close
}