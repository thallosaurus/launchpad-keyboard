use uinput::event::keyboard::{self, Key};

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
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