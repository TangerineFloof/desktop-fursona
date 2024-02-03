mod gl_context_wrapper;
mod stage_builder;

pub use stage_builder::StageBuilder;

use self::gl_context_wrapper::GlContextWrapper;
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin::surface::{Surface, SwapInterval, WindowSurface};
use glutin_winit::GlWindow;
use raw_window_handle::HasRawWindowHandle;
use std::num::NonZeroU32;
use winit::window::Window;

pub struct Stage {
    gl_surface: Surface<WindowSurface>,
    gl_context: GlContextWrapper,
    window: winit::window::Window,
}

impl Stage {
    pub fn new(window: Window, gl_config: &glutin::config::Config) -> Result<Stage, &'static str> {
        let raw_window_handle = Some(window.raw_window_handle());

        // XXX The display could be obtained from any object created by it, so we can
        // query it from the config.
        let gl_display = gl_config.display();

        let attrs = window.build_surface_attributes(Default::default());
        let gl_surface = unsafe {
            gl_config
                .display()
                .create_window_surface(&gl_config, &attrs)
                .unwrap()
        };

        Ok(Self {
            gl_context: GlContextWrapper::new(gl_config, &gl_display, raw_window_handle),
            gl_surface,
            window,
        })
    }

    pub fn draw(&mut self) {
        let gl_context = self.gl_context.make_current(&self.gl_surface);

        // Try setting vsync.
        if let Err(res) = self
            .gl_surface
            .set_swap_interval(&gl_context, SwapInterval::Wait(NonZeroU32::new(1).unwrap()))
        {
            eprintln!("Error setting vsync: {res:?}");
        }

        // self.renderer.draw();
        self.window.request_redraw();

        self.gl_surface.swap_buffers(&gl_context).unwrap();
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        let gl_context = self.gl_context.make_current(&self.gl_surface);

        // Some platforms like EGL require resizing GL surface to update the size
        // Notable platforms here are Wayland and macOS, other don't require it
        // and the function is no-op, but it's wise to resize it for portability
        // reasons.

        self.gl_surface.resize(
            &gl_context,
            NonZeroU32::new(width).unwrap(),
            NonZeroU32::new(height).unwrap(),
        );
        // self.renderer.resize(width as i32, height as i32);
    }
}
