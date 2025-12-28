use crate::message::{Message, MidiMessage};
use std::{
    error::Error,
    io::{Write, stdin, stdout},
};

use log::debug;
use midir::{MidiIO, MidiInput, MidiInputConnection, MidiOutput, MidiOutputConnection};
use tokio::sync::mpsc::{Receiver, Sender, channel};

pub fn connect_input(
    tx: Sender<Message>,
) -> Result<MidiInputConnection<Sender<Message>>, Box<dyn Error>> {
    let mut midi_in = MidiInput::new("midir forwarding input")?;
    midi_in.ignore(midir::Ignore::Time);
    let in_port = select_port(&midi_in, "midi input")?;

    Ok(midi_in.connect(
        &in_port,
        "lppro-gamecontroller",
        move |ts, msg, tx| {
            let parsed = Message::parse(ts, msg);
            //tx.blocking_send(parsed);
            if let Err(e) = tx.blocking_send(parsed) {
                eprintln!("Error receiving midi message: {}", e);
            }
        },
        tx,
    )?)
}

pub fn connect_output() -> Result<MidiOutputConnection, Box<dyn Error>> {
    let midi_out = MidiOutput::new("midir forwarding output")?;
    let out_port = select_port(&midi_out, "midi output")?;
    Ok(midi_out.connect(&out_port, "lppro-gamecontroller")?)
}

fn select_port<T: MidiIO>(midi_io: &T, descr: &str) -> Result<T::Port, Box<dyn Error>> {
    println!("Available {} ports:", descr);
    let midi_ports = midi_io.ports();
    for (i, p) in midi_ports.iter().enumerate() {
        println!("{}: {}", i, midi_io.port_name(p)?);
    }
    print!("Please select {} port: ", descr);
    stdout().flush()?;
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    let port = midi_ports
        .get(input.trim().parse::<usize>()?)
        .ok_or("Invalid port number")?;
    Ok(port.clone())
}
