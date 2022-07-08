use crate::app_trace; // TODO: how does this work?
use crate::platform;
use crate::platform::events::Event;

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
                    _ => (),
                }
            }

            self.platform.renderer.draw();
        }
    }
}
