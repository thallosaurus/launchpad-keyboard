use std::{collections::HashMap, sync::Arc};

use log::debug;
use midir::MidiOutputConnection;
use tokio::{sync::{Mutex, broadcast::{self, channel, error::RecvError}, mpsc::{Receiver, Sender}}, time::sleep};
//use uinput::event::keyboard;

use crate::{mapping::Config, message::{Message, MidiChannel, MidiMessage, MidiVelocity, Note}, virtual_input::Actions};

type ActiveActions = HashMap<Actions, MidiVelocity>;

pub async fn event_loop(config: Config, mut from_raw_device: Receiver<Message>, mut output_port: MidiOutputConnection) -> Result<(), RecvError> {
    let (tx, rx) = channel(1);
    let mut in_rx = tx.subscribe();
    let mut out_rx = tx.subscribe();

    ctrlc::set_handler(move || { tx.send(()).expect("could not send ctrlc sig on channel"); })
        .expect("error setting ctrlc handler");

    /*let mut device = uinput::default().unwrap()
        .name("launchpad-keyboard").unwrap()
        .event(uinput::event::Keyboard::All).unwrap()
        .create().unwrap();*/

    let actions_map = Arc::new(Mutex::new(ActiveActions::new()));
    let config = Arc::new(Mutex::new(config));

    let input_map = actions_map.clone();
    let _input_task = tokio::spawn(async move {
        let map = input_map;

        loop {
            tokio::select! {
                msg = from_raw_device.recv() => {
                    let msg_clone = msg.clone();
                    match msg {
                        Some(msg) => {
                            match msg.1 {
                                MidiMessage::NoteOn(ch, note, vel) => {
                                    debug!("{:?}", note);
                                    let action: Option<Actions> = note.into();
                                    if let Some(action) = action {
                                        let mut map = map.lock().await;

                                        if !map.contains_key(&action) {
                                            map.insert(action, vel);
                                        }
                                        drop(map);

                                        /*let key: keyboard::Key = action.into();
                                        device.press(&key).unwrap();
                                        device.synchronize().unwrap();*/
                                    }
                                },
                                MidiMessage::NoteOff(ch, note) => {
                                    debug!("{:?}", note);
                                    let action: Option<Actions> = note.into();
                                    if let Some(action) = action {
                                        
                                        let mut map = map.lock().await;
                                        if map.contains_key(&action) {
                                            map.remove(&action);
                                        }
                                        drop(map);
                                        
                                        /*
                                        let key: keyboard::Key = action.into();
                                        device.release(&key).unwrap();
                                        device.synchronize().unwrap();*/
                                    }
                                },
                                MidiMessage::AfterTouch(ch, note, vel) => {
                                    debug!("{:?}", note);
                                    let action: Option<Actions> = note.into();
                                    
                                    if let Some(action) = action {
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
                c = in_rx.recv() => {
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

        let config = out_config.lock().await;

        let mut last_len = 0;
        loop {
            tokio::select! {
                c = out_rx.recv() => {
                    debug!("closing output task");
                    break;
                }
            }
            
            //println!("Sent!");
        }

        // send all notes off
    });

    let join = tokio::join!(_input_task, _output_task);
    Ok(())
}

#[derive(Clone, Copy, Debug)]
enum BroadcastClose {
    Close
}