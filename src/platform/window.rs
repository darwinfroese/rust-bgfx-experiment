use crate::eng_trace;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::Window as SdlWindow;
use sdl2::{EventPump, Sdl};

use bgfx::*;
use bgfx_rs::bgfx;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

// TEMP: doing bgfx-rs stuff in here just to get it set up

pub struct Window {
    sdl_context: Sdl,
    sdl_window: SdlWindow,

    event_pump: EventPump,
}

impl Window {
    pub fn new<S>(title: S, width: u32, height: u32) -> Result<Window, Box<dyn std::error::Error>>
    where
        S: Into<String>,
    {
        eng_trace!("Creating a new window");

        let sdl_context = sdl2::init().map_err(|e| e.to_string())?;
        let video_subsystem = sdl_context.video().map_err(|e| e.to_string())?;

        let sdl_window = video_subsystem
            .window(&title.into(), width, height)
            .position_centered()
            .vulkan()
            .build()
            .map_err(|e| e.to_string())?;

        let event_pump = sdl_context.event_pump().map_err(|e| e.to_string())?;

        let window = Window {
            sdl_context,
            sdl_window,
            event_pump,
        };

        Ok(window)
    }

    pub fn run(&mut self) {
        let mut init = Init::new();
        init.type_r = RendererType::Vulkan;
        init.resolution.width = 800;
        init.resolution.height = 600;
        init.resolution.reset = ResetFlags::VSYNC.bits();
        init.platform_data = self.get_platform_data();

        if !bgfx::init(&init) {
            panic!("failed to init bgfx");
        }

        bgfx::set_debug(DebugFlags::TEXT.bits());
        bgfx::set_view_clear(
            0,
            ClearFlags::COLOR.bits() | ClearFlags::DEPTH.bits(),
            SetViewClearArgs {
                rgba: 0x103030ff,
                ..Default::default()
            },
        );

        'running: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }

            bgfx::set_view_rect(0, 0, 0, 800, 600);
            bgfx::touch(0);

            bgfx::dbg_text_clear(DbgTextClearArgs::default());

            bgfx::dbg_text(0, 1, 0x0f, "Color can be changed with ANSI \x1b[9;me\x1b[10;ms\x1b[11;mc\x1b[12;ma\x1b[13;mp\x1b[14;me\x1b[0m code too.");
            bgfx::dbg_text(80, 1, 0x0f, "\x1b[;0m    \x1b[;1m    \x1b[; 2m    \x1b[; 3m    \x1b[; 4m    \x1b[; 5m    \x1b[; 6m    \x1b[; 7m    \x1b[0m");
            bgfx::dbg_text(80, 2, 0x0f, "\x1b[;8m    \x1b[;9m    \x1b[;10m    \x1b[;11m    \x1b[;12m    \x1b[;13m    \x1b[;14m    \x1b[;15m    \x1b[0m");
            bgfx::dbg_text(
                0,
                4,
                0x3f,
                "Description: Initialization and debug text with bgfx-rs Rust API.",
            );

            bgfx::frame(false);
        }

        bgfx::shutdown();
    }

    fn get_platform_data(&self) -> PlatformData {
        let mut pd = PlatformData::new();

        let window = &self.sdl_window;

        match window.raw_window_handle() {
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
}
