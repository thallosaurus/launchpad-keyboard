use rdev::simulate;
use std::error::Error;

struct AgnosticBackend;

impl InputBackend for AgnosticBackend {
    fn process_on_action(&mut self, action: rdev::Key) {
        simulate(&rdev::EventType::KeyPress(action)).expect("error sending key");
    }
    
    fn process_off_action(&mut self, action: rdev::Key) {
        simulate(&rdev::EventType::KeyRelease(action)).expect("error sending key");
    }
}

pub trait InputBackend: Send + Sync {
    fn process_on_action(&mut self, action: rdev::Key);
    fn process_off_action(&mut self, action: rdev::Key);
}

pub fn create_backend() -> Result<Box<dyn InputBackend>, Box<dyn Error>> {
    Ok(Box::new(AgnosticBackend))
}