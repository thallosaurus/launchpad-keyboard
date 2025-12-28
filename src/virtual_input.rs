use log::info;
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

#[cfg(target_os = "linux")]
impl From<Actions> for keyboard::Key {
    fn from(value: Actions) -> Self {
        match value {
            Actions::Forward => Key::W,
            Actions::Backward => Key::A,
            Actions::Left => Key::S,
            Actions::Right => Key::D,
            Actions::A => Key::J,
            Actions::B => Key::K,
            Actions::Start => Key::Y,
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
}


pub trait InputBackend: Send + Sync {
    fn process_on_action(&mut self, action: Actions);
    fn process_off_action(&mut self, action: Actions);
}

pub fn create_backend() -> Result<Box<dyn InputBackend>, Box<dyn Error>> {
    #[cfg(not(target_os = "linux"))]
    { Ok(Box::new(UnsupportedInputStub)) }
    
    #[cfg(target_os = "linux")]
    {
        let mut device = uinput::default().unwrap()
            .name("launchpad-keyboard").unwrap()
            .event(uinput::event::Keyboard::All).unwrap()
            .create().unwrap();

        Ok(Box::new(LinuxBackend {
            device
        }))
    }
}