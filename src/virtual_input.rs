use rdev::simulate;
use std::error::Error;

struct AgnosticBackend;

impl InputBackend for AgnosticBackend {
    fn process_on_action(&mut self, action: rdev::Key) {
        //let k: rdev::Key = action.into();
        simulate(&rdev::EventType::KeyPress(action)).expect("error sending key");
        //simulate(event_type)
    }
    
    fn process_off_action(&mut self, action: rdev::Key) {
        //let k: rdev::Key = action.into();
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