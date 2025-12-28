use log::{debug, error, trace};

use crate::mapping::Actions;

#[derive(Debug)]
pub struct Message(u64, pub MidiMessage);

impl Message {
    pub fn parse(ts: u64, data: &[u8]) -> Self {
        Message(ts, MidiMessage::parse(data))
    }
}

pub type MidiChannel = u8;
pub type MidiVelocity = u8;
#[derive(Debug)]
pub enum MidiMessage {
    NoteOn(MidiChannel, Note, MidiVelocity),
    NoteOff(MidiChannel, Note),
    AfterTouch(MidiChannel, Note, MidiVelocity),
    Unknown
}


impl MidiMessage {
    pub fn parse(data: &[u8]) -> Self {
        trace!("{:?}", data);

        match data {
            [0x90..=0x9F, note, vel] if *vel > 0 => {
                let ch = data[0] - 0x90;
                Self::NoteOn(ch, Note::parse(*note), *vel)
            }
            [0x80..=0x8F, note, _] => {
                let ch = data[0] - 0x80;
                Self::NoteOff(ch, Note::parse(*note))

            }
            [0x90..=0x9F, note, 0] => {
                let ch = data[0] - 0x90;
                
                Self::NoteOff(ch, Note::parse(*note))
            }
            [0xA0..=0xAF, note, vel] => {
                let ch = data[0] - 0xA0;
                Self::AfterTouch(ch, Note::parse(*note), *vel)
            }
            _ => {
                error!("{:?}", data);
                Self::Unknown
            }
        }
    }
}

impl From<MidiMessage> for Vec<u8> {
    fn from(value: MidiMessage) -> Self {
        match value {
            MidiMessage::NoteOn(ch, note, vel) => vec![0x90 + ch, note.into(), vel],
            MidiMessage::NoteOff(ch, note) => vec![0x80 + ch, note.into(), 0],
            MidiMessage::AfterTouch(ch, note, vel) => vec![0xA0 + ch, note.into(), vel],
            MidiMessage::Unknown => todo!(),
        }
    }
}

#[derive(Debug)]
pub enum Note {
    C(u8),
    CS(u8),
    D(u8),
    DS(u8),
    E(u8),
    F(u8),
    FS(u8),
    G(u8),
    GS(u8),
    A(u8),
    AS(u8),
    B(u8)
}
impl Note {
    fn parse(data: u8) -> Self {
        let octave = data / 12;
        let key = data % 12;
        
        match key {
            0 => Self::C(octave),
            1 => Self::CS(octave),
            2 => Self::D(octave),
            3 => Self::DS(octave),
            4 => Self::E(octave),
            5 => Self::F(octave),
            6 => Self::FS(octave),
            7 => Self::G(octave),
            8 => Self::GS(octave),
            9 => Self::A(octave),
            10 => Self::AS(octave),
            11 => Self::B(octave),
            _ => panic!("unknown key")
        }
    }
}

impl From<Note> for u8 {
    fn from(value: Note) -> Self {
        let (oct, key) = match value {
            Note::C(oct) => (oct, 0),
            Note::CS(oct) => (oct, 1),
            Note::D(oct) => (oct, 2),
            Note::DS(oct) => (oct, 3),
            Note::E(oct) => (oct, 4),
            Note::F(oct) => (oct, 5),
            Note::FS(oct) => (oct, 6),
            Note::G(oct) => (oct, 7),
            Note::GS(oct) => (oct, 8),
            Note::A(oct) => (oct, 9),
            Note::AS(oct) => (oct, 10),
            Note::B(oct) => (oct, 11),
        };

        oct * 12 + key
    }
}

impl From<Note> for Option<Actions> {
    fn from(value: Note) -> Self {
        match value {
            Note::A(oct) if oct == 3 => {
                Some(Actions::Forward)
            },
            Note::F(oct) if oct == 3 => {
                Some(Actions::Backward)
            },
            Note::E(oct) if oct == 3 => {
                Some(Actions::Left)
            },
            Note::FS(oct) if oct == 3 => {
                Some(Actions::Right)
            }
            _ => {
                None
            }
        }
    }
}