pub mod ecs;
pub mod error;
pub mod events;
pub mod keys;
pub mod logging;
pub mod logging_macros;
pub mod renderer;
mod window;

pub struct Platform {
    pub renderer: renderer::Renderer,
    pub ecs: ecs::EcsManager,

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

        let ecs = ecs::EcsManager::new();

        Ok(Platform {
            window,
            renderer,
            ecs,
        })
    }

    // TODO: avoiding an event_bus for now, but with the ECS systems may want to emit events
    // (or components) onto the event_bus that will be responded to by downstream systems or
    // by the platform itself so we'll need an event_bus
    pub fn update(&mut self) -> Vec<events::Event> {
        self.ecs.run_systems();

        self.window.update()
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
