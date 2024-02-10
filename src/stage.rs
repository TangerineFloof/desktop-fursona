mod gl_context_wrapper;
mod stage_builder;

pub use stage_builder::StageBuilder;

use self::gl_context_wrapper::GlContextWrapper;
use crate::rendering::Renderer;
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin::surface::{Surface, SwapInterval, WindowSurface};
use glutin_winit::GlWindow;
use raw_window_handle::HasRawWindowHandle;
use std::num::NonZeroU32;
use winit::dpi::LogicalSize;
use winit::window::{CursorIcon, Window};

pub struct Stage {
    gl_surface: Surface<WindowSurface>,
    gl_context: GlContextWrapper,
    renderer: Renderer,
    window: winit::window::Window,
}

struct Point(f32, f32);

fn is_point_in_triangle(point: Point, triangle: (Point, Point, Point)) -> bool {
    /*
    """Returns True if the point is inside the triangle
    and returns False if it falls outside.
    - The argument *point* is a tuple with two elements
    containing the X,Y coordinates respectively.
    - The argument *triangle* is a tuple with three elements each
    element consisting of a tuple of X,Y coordinates.

    It works like this:
    Walk clockwise or counterclockwise around the triangle
    and project the point onto the segment we are crossing
    by using the dot product.
    Finally, check that the vector created is on the same side
    for each of the triangle's segments.
    """
     */

    let x = point.0;
    let y = point.1;
    let ax = triangle.0 .0;
    let ay = triangle.0 .1;
    let bx = triangle.1 .0;
    let by = triangle.1 .1;
    let cx = triangle.2 .0;
    let cy = triangle.2 .1;

    // # Segment A to B
    let side_1 = (x - bx) * (ay - by) - (ax - bx) * (y - by);
    // # Segment B to C
    let side_2 = (x - cx) * (by - cy) - (bx - cx) * (y - cy);
    // # Segment C to A
    let side_3 = (x - ax) * (cy - ay) - (cx - ax) * (y - ay);
    // # All the signs must be positive or all negative
    ((side_1 < 0.0) == (side_2 < 0.0)) == (side_3 < 0.0)
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

        let mut gl_context = GlContextWrapper::new(gl_config, &gl_display, raw_window_handle);
        let renderer = {
            // We need our new context to be current in order to set up the
            // programs
            let _current = gl_context.make_current(&gl_surface);
            Renderer::new(&gl_display)
        };

        Ok(Self {
            gl_context,
            gl_surface,
            renderer,
            window,
        })
    }

    pub fn on_mouse_over(&self, x: u32, y: u32) {
        let scale_factor = self.window.scale_factor();
        let size: LogicalSize<f32> = self.window.inner_size().to_logical(scale_factor);
        let half_width = (size.width as f32) / 2.0;
        let half_height = (size.height as f32) / 2.0;

        // Convert to renderer coordinate system
        let rx = (x as f32 - half_width) / half_width;
        let ry = (half_height - y as f32) / half_height;

        let inside = is_point_in_triangle(
            Point(rx, ry),
            (Point(-0.5, -0.5), Point(0.0, 0.5), Point(0.5, -0.5)),
        );

        self.window.set_cursor_hittest(inside).unwrap();
        if inside {
            // We need to focus the window in order for the cursor change to
            // actually show up
            self.window.focus_window();
            self.window.set_cursor_icon(CursorIcon::Pointer);
        } else {
            // TODO: Restore focus to the window we stole it from? Is that even possible?
            self.window.set_cursor_icon(CursorIcon::Default);
        }
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

        self.renderer.draw();
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
        self.renderer.resize(width, height);
    }
}
