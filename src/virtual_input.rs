use log::info;
use rdev::{Key, simulate};
use std::error::Error;

#[cfg(target_os = "linux")]
use uinput::{Device, event::keyboard::{self, Key}};

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

/*#[cfg(target_os = "linux")]
impl From<Actions> for keyboard::Key {
    fn from(value: Actions) -> Self {
        match value {
            Actions::Forward => Key::W,
            Actions::Backward => Key::A,
            Actions::Left => Key::S,
            Actions::Right => Key::D,
            Actions::A => Key::J,
            Actions::B => Key::K,
            Actions::Start => Key::Esc,
            Actions::Select => Key::X,
        }
    }
}

#[cfg(target_os = "linux")]
struct LinuxBackend {
    device: Device
}

#[cfg(target_os = "linux")]
impl InputBackend for LinuxBackend {
    fn process_on_action(&mut self, action: Actions) {
        info!("On: {:?}", action);
        let ev: keyboard::Key = action.into();
        self.device.press(&ev).expect("error while pressing key");
        self.device.synchronize().expect("error while synchronizing device");
    }
    
    fn process_off_action(&mut self, action: Actions) {
        info!("Off: {:?}", action);
        let ev: keyboard::Key = action.into();
        self.device.release(&ev).expect("error while pressing key");
        self.device.synchronize().expect("error while synchronizing device");

    }
}

#[cfg(not(target_os = "linux"))]
struct UnsupportedInputStub;

#[cfg(not(target_os = "linux"))]
impl InputBackend for UnsupportedInputStub {
    fn process_on_action(&mut self, action: Actions) {
        info!("{:?}", action);
    }
    
    fn process_off_action(&mut self, action: Actions) {
        info!("{:?}", action);
    }
}*/

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
    #[cfg(not(target_os = "linux"))]
    { Ok(Box::new(AgnosticBackend)) }
    
    #[cfg(target_os = "linux")]
    {
        let device = uinput::default().unwrap()
            .name("launchpad-keyboard").unwrap()
            .event(uinput::event::Keyboard::All).unwrap()
            .create().unwrap();

        Ok(Box::new(LinuxBackend {
            device
        }))
    }
}