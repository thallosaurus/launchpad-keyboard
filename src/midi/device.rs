use std::{
    error::Error,
    io::{Write, stdin, stdout},
};

use midir::{MidiIO, MidiInput, MidiInputConnection, MidiOutput, MidiOutputConnection};
use tokio::sync::mpsc::Sender;

use crate::midi::message::Message;

/// Connect to the MIDI Input Device
pub fn connect_input(
    name: Option<String>,
    tx: Sender<Message>,
) -> Result<MidiInputConnection<Sender<Message>>, Box<dyn Error>> {
    let mut midi_in = MidiInput::new("midir forwarding input")?;
    //midi_in.ignore(midir::Ignore::Time);
    midi_in.ignore(midir::Ignore::None);

    let in_port = if let Some(name) = name {
        select_port_by_name(&midi_in, name)?
    } else {
        select_port(&midi_in, "midi input")?
    };

    Ok(midi_in.connect(
        &in_port,
        "lppro-gamecontroller",
        move |ts, msg, tx| {
            let msg = Vec::from(msg);
            let parsed = Message::parse(ts, msg);

            // send blocking, because we are in a synchronous thread
            if let Err(e) = tx.blocking_send(parsed) {
                eprintln!("Error receiving midi message: {}", e);
            }
        },
        tx,
    )?)
}

/// Connect to MIDI Output
pub fn connect_output(name: Option<String>) -> Result<MidiOutputConnection, Box<dyn Error>> {
    let midi_out = MidiOutput::new("midir forwarding output")?;
    let out_port = if let Some(name) = name {
        select_port_by_name(&midi_out, name)?
    } else {
        select_port(&midi_out, "midi input")?
    };

    Ok(midi_out.connect(&out_port, "lppro-gamecontroller")?)
}

/// Prompts the user to select the device
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

/// Select MIDI Device by Name
fn select_port_by_name<T: MidiIO>(midi_io: &T, search: String) -> Result<T::Port, Box<dyn Error>> {
    let midi_ports = midi_io.ports();

    let possible: Vec<T::Port> = midi_ports.iter().enumerate().filter(|(_, v)| {
        let name = midi_io.port_name(v).unwrap();
        return name.trim() == search.trim();
    }).map(|(_, p)| {
        p.clone()
    }).collect();

    Ok(possible.first().cloned().unwrap())
}