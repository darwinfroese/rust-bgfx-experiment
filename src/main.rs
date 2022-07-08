mod application;
mod platform;

fn main() {
    platform::logging::init_logging();

    app_trace!("Launching application");

    let platform = platform::PlatformBuilder::new("Rust BGFX Experminte", 1280, 720)
        .build()
        .unwrap();

    let mut application = application::Application::new(platform);

    application.run();
}
