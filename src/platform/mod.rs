pub mod error;
pub mod logging;
pub mod logging_macros;
pub mod renderer;
mod window;

pub struct Platform {
    pub renderer: renderer::Renderer,

    window: window::Window,
}

impl Platform {
    pub fn new(settings: &PlatformBuilder) -> error::Result<Platform> {
        let window = window::Window::new(
            &settings.title,
            settings.window_width,
            settings.window_height,
        )?;

        let renderer = renderer::RendererBuilder::new(window.get_platform_data()).build()?;

        Ok(Platform { window, renderer })
    }
}

pub struct PlatformBuilder {
    pub title: String,
    pub window_width: u32,
    pub window_height: u32,
}

impl PlatformBuilder {
    pub fn new<S>(title: S, window_width: u32, window_height: u32) -> PlatformBuilder
    where
        S: Into<String>,
    {
        PlatformBuilder {
            title: title.into(),
            window_width,
            window_height,

            ..PlatformBuilder::default()
        }
    }

    pub fn build(&self) -> error::Result<Platform> {
        Platform::new(self)
    }
}

impl Default for PlatformBuilder {
    fn default() -> PlatformBuilder {
        PlatformBuilder {
            title: "Rust BGFX Experiment".into(),
            window_width: 800,
            window_height: 600,
        }
    }
}
