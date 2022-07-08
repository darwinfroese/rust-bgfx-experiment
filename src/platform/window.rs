use crate::eng_trace;
use crate::platform::error;
use crate::platform::events;

use sdl2::video::Window as SdlWindow;
use sdl2::{EventPump, Sdl};

use bgfx::PlatformData;
use bgfx_rs::bgfx;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

pub(crate) struct Window {
    sdl_context: Sdl,
    sdl_window: SdlWindow,

    event_pump: EventPump,
}

impl Window {
    pub(crate) fn new<S>(title: S, width: u32, height: u32) -> error::Result<Window>
    where
        S: Into<String>,
    {
        eng_trace!("Creating a new window");

        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let sdl_window = video_subsystem
            .window(&title.into(), width, height)
            .position_centered()
            .vulkan()
            .build()?;

        let event_pump = sdl_context.event_pump()?;

        let window = Window {
            sdl_context,
            sdl_window,
            event_pump,
        };

        Ok(window)
    }

    pub fn update(&mut self) -> Vec<events::Event> {
        let mut events = Vec::new();
        for event in self.event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => {
                    events.push(events::Event::Quit);
                }
                sdl2::event::Event::KeyDown { .. } => {
                    events.push(events::Event::Quit);
                }
                _ => (),
            }
        }

        events
    }

    pub fn get_platform_data(&self) -> PlatformData {
        let mut pd = PlatformData::new();

        match self.sdl_window.raw_window_handle() {
            #[cfg(any(
                target_os = "linux",
                target_os = "dragonfly",
                target_os = "freebsd",
                target_os = "netbsd",
                target_os = "openbsd"
            ))]
            RawWindowHandle::Xlib(data) => {
                pd.nwh = data.window as *mut _;
                pd.ndt = data.display as *mut _;
            }
            #[cfg(any(
                target_os = "linux",
                target_os = "dragonfly",
                target_os = "freebsd",
                target_os = "netbsd",
                target_os = "openbsd"
            ))]
            RawWindowHandle::Wayland(data) => {
                pd.ndt = data.surface; // same as window, on wayland there ins't a concept of windows
                pd.nwh = data.display;
            }

            #[cfg(target_os = "macos")]
            RawWindowHandle::MacOS(data) => {
                pd.nwh = data.ns_window;
            }
            #[cfg(target_os = "windows")]
            RawWindowHandle::Windows(data) => {
                pd.nwh = data.hwnd;
            }
            _ => panic!("Unsupported Window Manager"),
        }

        return pd;
    }

    fn create_key_event(self) {}

    fn create_quit_event(self) {}
}
