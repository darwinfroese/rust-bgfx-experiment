use crate::eng_trace;
use crate::platform::error;

use bgfx::*;
use bgfx_rs::bgfx;

pub struct Renderer {}

impl Renderer {
    pub fn new() -> Renderer {
        bgfx::set_debug(DebugFlags::TEXT.bits());
        bgfx::set_view_clear(
            0,
            ClearFlags::COLOR.bits() | ClearFlags::DEPTH.bits(),
            SetViewClearArgs {
                rgba: 0x103030ff,
                ..Default::default()
            },
        );

        bgfx::set_view_rect(0, 0, 0, 1280, 720);

        Renderer {}
    }

    pub fn draw(&self) {
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

        bgfx::touch(0);

        bgfx::frame(false);
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
