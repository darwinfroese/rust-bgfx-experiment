use crate::app_trace;
use crate::platform;
use crate::platform::events::Event;
use crate::platform::keys;

pub struct Application {
    platform: platform::Platform,

    is_running: bool,
}

impl Application {
    pub fn new(platform: platform::Platform) -> Application {
        app_trace!("Creating an application");

        Application {
            is_running: true,
            platform,
        }
    }

    pub fn run(&mut self) {
        app_trace!("Running application");

        while self.is_running {
            let events = self.platform.update();

            for event in events {
                match event {
                    Event::Quit => {
                        app_trace!("Quit event received. Exiting the application");
                        self.is_running = false; // TODO: should the rest of the events be processed?
                    }
                    Event::KeyPressed {
                        key: keys::Key::Escape,
                    } => {
                        app_trace!("Escape pressed. Exiting the application");
                        self.is_running = false;
                    }
                    Event::KeyPressed {
                        key: keys::Key::P, // TODO: map this to something that wouldn't be used in game
                    } => {
                        app_trace!("Toggling renderer profiling.");
                        self.platform.renderer.toggle_profiling();
                    }
                    _ => (),
                }
            }

            // TODO: These should be put into the ECS
            self.platform.renderer.draw();
            self.platform
                .renderer
                .draw_text_debug("BGFX Experiment in Rust!");
        }
    }
}
