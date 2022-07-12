mod application;
mod platform;

fn main() {
    platform::logging::init_logging();

    app_trace!("Launching application");

    let platform = platform::PlatformBuilder::new("Rust BGFX Experiment", 1280, 720)
        .build()
        .unwrap();

    let mut application = application::Application::new(platform);

    application.run();
}
