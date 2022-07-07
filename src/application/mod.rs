use crate::platform;

pub struct Application {
    platform: platform::Platform,

    is_running: bool,
}

impl Application {
    pub fn new(platform: platform::Platform) -> Application {
        Application {
            is_running: true,
            platform,
        }
    }

    pub fn run(&self) {
        while self.is_running {}
    }
}
