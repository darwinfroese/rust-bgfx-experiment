use crate::platform::keys;

#[derive(Debug)]
pub enum Event {
    Quit,
    KeyPressed { key: keys::Key },
}
