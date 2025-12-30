use log::debug;
use once_cell::sync::Lazy;
use serde::{
    Deserialize, Serialize,
    de::{self, Visitor},
};
use std::{
    collections::HashMap,
    fmt::{self, Display},
    sync::Mutex,
};

pub static MAPPING: Lazy<Mutex<HashMap<MidiNote, rdev::Key>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub fn set_mapping(m: HashMap<MidiNote, rdev::Key>) {
    for (_, (m, ac)) in m.iter().enumerate() {
        debug!("MAPPING: {:?} = {:?}", ac, m);

        {
            let mut mapping = MAPPING.lock().unwrap();
            mapping.insert(*m, *ac);
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, Serialize)]
pub enum MidiNote {
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
    B(u8),
}

impl<'de> Deserialize<'de> for MidiNote {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct NoteVisitor;

        impl<'de> Visitor<'de> for NoteVisitor {
            type Value = MidiNote;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "a MIDI note like C5 or FS3")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                MidiNote::parse(v).map_err(E::custom)
            }
        }

        deserializer.deserialize_str(NoteVisitor)
    }
}

impl MidiNote {
    fn parse(value: &str) -> Result<Self, MappingError> {
        let value = value.trim();
        let (note_str, octave_str) = value
            .chars()
            .partition::<String, _>(|c| !c.is_ascii_digit() && *c != '-');

        // Handle Octave, inklusive negatives
        let octave: u8 = octave_str
            .parse()
            .unwrap_or_else(|_| panic!("Invalid octave in note string: {}", value));

        //let octave = octave  2;

        match note_str.as_str() {
            "C" => Ok(MidiNote::C(octave)),
            "C#" | "CS" => Ok(MidiNote::CS(octave)),
            "D" => Ok(MidiNote::D(octave)),
            "D#" | "DS" => Ok(MidiNote::DS(octave)),
            "E" => Ok(MidiNote::E(octave)),
            "F" => Ok(MidiNote::F(octave)),
            "F#" | "FS" => Ok(MidiNote::FS(octave)),
            "G" => Ok(MidiNote::G(octave)),
            "G#" | "GS" => Ok(MidiNote::GS(octave)),
            "A" => Ok(MidiNote::A(octave)),
            "A#" | "AS" => Ok(MidiNote::AS(octave)),
            "B" => Ok(MidiNote::B(octave)),
            _ => Err(MappingError::UnknownMidiKey), //_ => panic!("Unknown note string: {}", value),
        }
    }
}

impl From<u8> for MidiNote {
    fn from(value: u8) -> Self {
        let octave = value / 12;
        let key = value % 12;

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
            _ => panic!("unknown key"),
        }
    }
}

impl From<MidiNote> for u8 {
    fn from(value: MidiNote) -> Self {
        let (oct, key) = match value {
            MidiNote::C(oct) => (oct, 0),
            MidiNote::CS(oct) => (oct, 1),
            MidiNote::D(oct) => (oct, 2),
            MidiNote::DS(oct) => (oct, 3),
            MidiNote::E(oct) => (oct, 4),
            MidiNote::F(oct) => (oct, 5),
            MidiNote::FS(oct) => (oct, 6),
            MidiNote::G(oct) => (oct, 7),
            MidiNote::GS(oct) => (oct, 8),
            MidiNote::A(oct) => (oct, 9),
            MidiNote::AS(oct) => (oct, 10),
            MidiNote::B(oct) => (oct, 11),
        };

        oct * 12 + key
    }
}

impl From<MidiNote> for Option<rdev::Key> {
    fn from(value: MidiNote) -> Self {
        let m = MAPPING.lock().unwrap();

        m.get(&value.into()).cloned()
    }
}

// MARK: Errors
#[derive(Debug)]
enum MappingError {
    UnknownMidiKey,
}

impl std::error::Error for MappingError {}

impl Display for MappingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parsing() {
        assert_eq!(true, false); // TODO Implement
    }

    #[test]
    fn test_unsupported_notes_fail() {
        assert_eq!(true, false); // TODO Implement
    }
}
