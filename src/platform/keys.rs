#[derive(Debug, Clone, Copy)]
pub enum Key {
    Escape,

    P,

    Unimplemented,
}

#[macro_export]
macro_rules! to_key {
    ($x: expr) => {{
        match $x {
            sdl2::keyboard::Keycode::Escape => crate::platform::keys::Key::Escape,
            sdl2::keyboard::Keycode::P => crate::platform::keys::Key::P,
            _ => crate::platform::keys::Key::Unimplemented,
        }
    }};
}
