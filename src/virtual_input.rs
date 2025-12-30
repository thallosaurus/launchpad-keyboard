use log::info;
use rdev::simulate;
use std::{error::Error, process::Command};

use crate::config::Action;

struct AgnosticBackend;

impl InputBackend for AgnosticBackend {
    fn process_on_action(&mut self, action: Action) {
        match action {
            Action::Key(key) => simulate(&rdev::EventType::KeyPress(key)).expect("error sending key"),
            Action::Shell { press: Some(press), release: _ } => command_runner(press),
            _ => {}
        }
    }
    
    fn process_off_action(&mut self, action: Action) {
        match action {
            Action::Key(key) => simulate(&rdev::EventType::KeyRelease(key)).expect("error sending key"),
            Action::Shell { press: _, release: Some(release) } => command_runner(release),
            _ => {}
        }
    }
}

pub trait InputBackend: Send + Sync {
    fn process_on_action(&mut self, action: Action);
    fn process_off_action(&mut self, action: Action);
}

pub fn create_backend() -> Result<Box<dyn InputBackend>, Box<dyn Error>> {
    Ok(Box::new(AgnosticBackend))
}

fn command_runner(cmd: String) {
    info!("running command: {}", cmd);
    let mut c = Command::new(cmd);
    c.spawn().unwrap();
}