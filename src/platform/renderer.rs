use crate::eng_trace;
use crate::platform::error;

use bgfx::*;
use bgfx_rs::bgfx;

pub struct Renderer {
    profiling: bool,
}

impl Renderer {
    pub fn new() -> Renderer {
        bgfx::set_view_clear(
            0,
            ClearFlags::COLOR.bits() | ClearFlags::DEPTH.bits(),
            SetViewClearArgs {
                rgba: 0x103030ff,
                ..Default::default()
            },
        );

        bgfx::set_view_rect(0, 0, 0, 1280, 720);

        if cfg!(debug_assertions) {
            bgfx::set_debug(DebugFlags::TEXT.bits());
        }

        Renderer { profiling: false }
    }

    #[cfg(debug_assertions)]
    pub fn toggle_profiling(&mut self) {
        self.profiling = !self.profiling;

        if self.profiling {
            bgfx::set_debug(
                DebugFlags::PROFILER.bits() | DebugFlags::STATS.bits() | DebugFlags::TEXT.bits(),
            );
        } else {
            bgfx::set_debug(DebugFlags::TEXT.bits());
        }
    }

    #[cfg(not(debug_assertions))]
    pub fn toggle_profiling(&self) {}

    pub fn draw(&self) {
        bgfx::touch(0); // "clear" screen

        bgfx::frame(false); // swap buffers (capture frame with graphics debugger)
    }

    #[cfg(debug_assertions)]
    pub fn draw_text_debug<S>(&self, text: S)
    where
        S: Into<String>,
    {
        bgfx::dbg_text_clear(DbgTextClearArgs::default());
        bgfx::dbg_text(0, 1, 0x0f, &text.into());
    }

    #[cfg(not(debug_assertions))]
    pub fn draw_text_debug<S>(&self, text: S)
    where
        S: Into<String>,
    {
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        bgfx::shutdown();
    }
}

pub struct RendererBuilder {}

impl RendererBuilder {
    pub fn new(platform_data: PlatformData) -> RendererBuilder {
        eng_trace!("Creating a renderer");

        let mut init = Init::new();
        init.type_r = RendererType::Vulkan;
        init.resolution.width = 1280;
        init.resolution.height = 720;
        init.resolution.reset = ResetFlags::VSYNC.bits();
        init.platform_data = platform_data;

        if !bgfx::init(&init) {
            panic!("failed to init bgfx");
        }

        RendererBuilder {}
    }

    pub fn build(&self) -> error::Result<Renderer> {
        Ok(Renderer::new())
    }
}
