mod stage_builder;
mod viewport;
mod viewport_point;

use glium::Display;
use glutin::context::{ContextApi, ContextAttributesBuilder, Version};
pub use stage_builder::StageBuilder;

use crate::rendering::{Renderer, RendererCoord};
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin::surface::WindowSurface;
use glutin_winit::GlWindow;
use raw_window_handle::HasRawWindowHandle;
use std::rc::Rc;
use winit::window::{CursorIcon, Window};

use viewport::Viewport;
pub use viewport_point::ViewportPoint;

pub struct Stage {
    pub display: Display<WindowSurface>,
    pub viewport: Viewport,
    window: Rc<Window>,
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

        // The context creation part. It can be created before surface and that's how
        // it's expected in multithreaded + multiwindow operation mode, since you
        // can send NotCurrentContext, but not Surface.
        let context_attributes = ContextAttributesBuilder::new().build(raw_window_handle);

        // Since glutin by default tries to create OpenGL core context, which may not be
        // present we should try gles.
        let fallback_context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::Gles(None))
            .build(raw_window_handle);

        // There are also some old devices that support neither modern OpenGL nor GLES.
        // To support these we can try and create a 2.1 context.
        let legacy_context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::OpenGl(Some(Version::new(2, 1))))
            .build(raw_window_handle);

        let not_current_gl_context = unsafe {
            gl_display
                .create_context(&gl_config, &context_attributes)
                .unwrap_or_else(|_| {
                    gl_display
                        .create_context(&gl_config, &fallback_context_attributes)
                        .unwrap_or_else(|_| {
                            gl_display
                                .create_context(&gl_config, &legacy_context_attributes)
                                .expect("failed to create context")
                        })
                })
        };

        let display = glium::Display::new(
            not_current_gl_context.make_current(&gl_surface).unwrap(),
            gl_surface,
        )
        .unwrap();

        let window = Rc::new(window);

        Ok(Self {
            display,
            viewport: Viewport::new(window.clone()),
            window,
        })
    }

    pub fn on_mouse_over(&self, x: u32, y: u32) {
        let RendererCoord { x: rx, y: ry } = self
            .viewport
            .convert_point_to_renderer_coord(ViewportPoint { x, y });

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

    pub fn draw(&mut self, renderers: &Vec<Box<dyn Renderer>>) {
        let mut frame = self.display.draw();
        for renderer in renderers {
            renderer.draw(&mut frame);
        }

        self.window.request_redraw();
        frame.finish().unwrap();
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.display.resize((width, height));
    }
}
