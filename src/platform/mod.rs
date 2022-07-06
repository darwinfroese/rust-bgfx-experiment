pub mod logging;

pub mod logging_macros;
pub mod renderer;
pub mod window;
use std::error::Error;

pub struct Platform {
    pub window: window::Window,
    pub renderer: renderer::Renderer,
}

impl Platform {
    pub fn new(settings: &PlatformBuilder) -> Result<Platform, Box<dyn Error>> {
        let window = window::Window::new(
            &settings.title,
            settings.window_width,
            settings.window_height,
        )?;

        let renderer = renderer::Renderer::new();

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

    pub fn title<S>(&mut self, title: S) -> &mut PlatformBuilder
    where
        S: Into<String>,
    {
        self.title = title.into();
        self
    }

    pub fn size(&mut self, window_width: u32, window_height: u32) -> &mut PlatformBuilder {
        self.window_width = window_width;
        self.window_height = window_height;
        self
    }

    pub fn build(&self) -> Result<Platform, Box<dyn std::error::Error>> {
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
