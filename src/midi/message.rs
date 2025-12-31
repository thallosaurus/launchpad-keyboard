
use log::error;

use crate::midi::note::MidiNote;

//The whole message that was sent from the MIDI Device
#[derive(Debug, Clone, Copy)]
pub struct Message(u64, pub MidiMessage);

impl From<(u64, Vec<u8>)> for Message {
    fn from(value: (u64, Vec<u8>)) -> Self {
        Message(value.0, value.1.into())
    }
}

pub type MidiChannel = u8;
pub type MidiVelocity = u8;

#[derive(Debug, Clone, Copy)]
pub enum MidiMessage {
    NoteOn(MidiChannel, MidiNote, MidiVelocity),
    NoteOff(MidiChannel, MidiNote),
    AfterTouch(MidiChannel, MidiNote, MidiVelocity),
    Clock,
    Unknown,
}

impl From<MidiMessage> for Vec<u8> {
    fn from(value: MidiMessage) -> Self {
        match value {
            MidiMessage::NoteOn(ch, note, vel) => vec![0x90 + ch, note.into(), vel],
            MidiMessage::NoteOff(ch, note) => vec![0x80 + ch, note.into(), 0],
            MidiMessage::AfterTouch(ch, note, vel) => vec![0xA0 + ch, note.into(), vel],
            MidiMessage::Unknown => todo!(),
            MidiMessage::Clock => todo!(),
        }
    }
}

impl From<Vec<u8>> for MidiMessage {
    fn from(data: Vec<u8>) -> Self {
        //trace!("{:?}", data);

        match data.as_slice() {
            [0x90..=0x9F, note, vel] if *vel > 0 => {
                let ch = data[0] - 0x90;
                let n = (*note).into();
                Self::NoteOn(ch, n, *vel)
            }
            [0x80..=0x8F, note, _] => {
                let ch = data[0] - 0x80;
                let n = (*note).into();
                Self::NoteOff(ch, n)
            }
            [0x90..=0x9F, note, 0] => {
                let ch = data[0] - 0x90;
                let n = (*note).into();
                Self::NoteOff(ch, n)
            }
            [0xA0..=0xAF, note, vel] => {
                let ch = data[0] - 0xA0;
                let n = (*note).into();
                Self::AfterTouch(ch, n, *vel)
            },
            [0xF8] => {
                Self::Clock
            }
            _ => {
                error!("{:?}", data);
                Self::Unknown
            }
        }
    }
}