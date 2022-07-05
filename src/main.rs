mod platform;

fn main() {
    platform::logging::init_logging();

    app_trace!("Launching application");

    platform::window::new_window();
}
