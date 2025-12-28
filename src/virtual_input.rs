use rdev::simulate;
use std::error::Error;

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
pub enum Actions {
    Forward,
    Backward,
    Left,
    Right,
    A,
    B,
    Start,
    Select,
}

impl From<&str> for Actions {
    fn from(value: &str) -> Self {
        match value {
            "forward" => Self::Forward,
            "backward" => Self::Backward,
            "left" => Self::Left,
            "right" => Self::Right,
            "a" => Self::A,
            "b" => Self::B,
            "start" => Self::Start,
            "select" => Self::Select,
            _ => panic!("unknown mapping")
        }
    }
}

struct AgnosticBackend;

impl InputBackend for AgnosticBackend {
    fn process_on_action(&mut self, action: Actions) {
        let k: rdev::Key = action.into();
        simulate(&rdev::EventType::KeyPress(k)).expect("error sending key");
        //simulate(event_type)
    }
    
    fn process_off_action(&mut self, action: Actions) {
        let k: rdev::Key = action.into();
        simulate(&rdev::EventType::KeyRelease(k)).expect("error sending key");
    }
}

impl From<Actions> for rdev::Key {
    fn from(value: Actions) -> Self {
        match value {
            Actions::Forward => rdev::Key::KeyW,
            Actions::Backward => rdev::Key::KeyA,
            Actions::Left => rdev::Key::KeyS,
            Actions::Right => rdev::Key::KeyD,
            Actions::A => rdev::Key::KeyJ,
            Actions::B => rdev::Key::KeyK,
            Actions::Start => rdev::Key::Escape,
            Actions::Select => rdev::Key::KeyX,
        }
    }
}

pub trait InputBackend: Send + Sync {
    fn process_on_action(&mut self, action: Actions);
    fn process_off_action(&mut self, action: Actions);
}

pub fn create_backend() -> Result<Box<dyn InputBackend>, Box<dyn Error>> {
    Ok(Box::new(AgnosticBackend))
}