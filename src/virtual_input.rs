#[cfg(target_os = "linux")]
use uinput::event::keyboard::{self, Key};

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

struct VirtualInput {

}

impl VirtualInput {
/*    pub fn init() {
        let mut device = uinput::default().unwrap()
            .name("launchpad-keyboard").unwrap()
            .event(uinput::event::Keyboard::All).unwrap()
            .create().unwrap();
    }
    */
}

#[cfg(target_os = "linux")]
use uinput::Key;

#[cfg(target_os = "linux")]
impl VirtInput for VirtualInput {
    fn send_action(action: Actions) {

    }
}

#[cfg(not(target_os = "linux"))]
impl VirtInput for VirtualInput {
    fn send_action(action: Actions) {
        println!("{:?}", action);
    }
    
    fn new() -> Self {
        todo!()
    }
}


trait VirtInput {
    fn new() -> Self;
    fn send_action(action: Actions);
}