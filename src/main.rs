mod platform;

use crate::platform::window::Window;


fn main() {
    platform::logging::init_logging();

    app_trace!("Launching application");

    let mut window = Window::new(String::from("Rust BGFX Experiment - Game"), 800, 600).map_err(|e| e.to_string()).unwrap();

    window.run();
}
