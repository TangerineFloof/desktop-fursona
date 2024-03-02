mod viewport;
mod viewport_point;
mod viewport_rect;

use glium::{Blend, Display, DrawParameters, Surface};
use glutin::config::{Config, ConfigTemplateBuilder};
use glutin::context::{ContextApi, ContextAttributesBuilder, Version};

use crate::fursona::FursonaInstance;
use crate::rendering::{Color, Renderer, RendererCoord};
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin::surface::WindowSurface;
use glutin_winit::{DisplayBuilder, GlWindow};
use raw_window_handle::HasRawWindowHandle;
use std::rc::Rc;
use winit::event_loop::EventLoop;
#[cfg(target_os = "macos")]
use winit::platform::macos::WindowExtMacOS;
use winit::window::{CursorIcon, Window, WindowBuilder, WindowLevel};

pub use viewport::Viewport;
pub use viewport_point::ViewportPoint;
pub use viewport_rect::ViewportRect;

const DEBUG_COLORS: [Color; 3] = [
    Color(1.0, 0.0, 0.0, 1.0),
    Color(0.0, 1.0, 0.0, 1.0),
    Color(0.0, 0.0, 1.0, 1.0),
];

pub struct Stage {
    pub display: Display<WindowSurface>,
    pub viewport: Viewport,
    renderer: Renderer,
    window: Rc<Window>,
    debug_mode: bool,
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

fn init_opengl(event_loop: &EventLoop<()>, window_builder: WindowBuilder) -> (Window, Config) {
    // The template will match only the configurations supporting rendering
    // to windows.
    //
    // XXX We force transparency only on macOS, given that EGL on X11 doesn't
    // have it, but we still want to show window. The macOS situation is like
    // that, because we can query only one config at a time on it, but all
    // normal platforms will return multiple configs, so we can find the config
    // with transparency ourselves inside the `reduce`.
    let template = ConfigTemplateBuilder::new()
        .with_alpha_size(8)
        .with_transparency(true);

    let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder));

    let (window, gl_config) = display_builder
        .build(event_loop, template, |configs| {
            // Find the config with the maximum number of samples, so our triangle will
            // be smooth.
            configs
                .reduce(|accum, config| {
                    let transparency_check = config.supports_transparency().unwrap_or(false)
                        & !accum.supports_transparency().unwrap_or(false);

                    if transparency_check || config.num_samples() > accum.num_samples() {
                        config
                    } else {
                        accum
                    }
                })
                .unwrap()
        })
        .unwrap();

    println!("Picked a config with {} samples", gl_config.num_samples());

    if let Some(trans) = gl_config.supports_transparency() {
        println!("picked transparent? {}", trans);
    } else {
        println!("oh no...");
    }

    (window.unwrap(), gl_config)
}

impl Stage {
    pub fn new(event_loop: &EventLoop<()>) -> Result<Stage, &'static str> {
        let window_builder = WindowBuilder::new()
            .with_transparent(true)
            .with_decorations(false)
            .with_window_level(WindowLevel::AlwaysOnTop);

        let (window, gl_config) = init_opengl(event_loop, window_builder);

        #[cfg(target_os = "macos")]
        window.set_simple_fullscreen(true);

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
        let renderer = Renderer::new(&display);

        Ok(Self {
            display,
            renderer,
            viewport: Viewport::new(window.clone()),
            window,
            debug_mode: false,
        })
    }

    pub fn set_debug_mode(&mut self, enabled: bool) {
        self.debug_mode = enabled;
    }

    pub fn on_mouse_over(&self, point: ViewportPoint) {
        let RendererCoord { x: rx, y: ry } = self.viewport.convert_point_to_renderer_coord(point);

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

    pub fn draw<'a, I: Iterator<Item = &'a FursonaInstance>>(&mut self, instances: I) {
        let mut frame = self.display.draw();
        frame.clear_all((0.0, 0.0, 0.0, 0.0), 0.0, 0);

        let draw_parameters = DrawParameters {
            blend: Blend::alpha_blending(),
            ..Default::default()
        };

        for (index, instance) in instances.enumerate() {
            let rect = self.viewport.convert_rect(instance.bounding_box());

            let debug_color = &DEBUG_COLORS[index % DEBUG_COLORS.len()];

            if self.debug_mode {
                self.renderer.fill_rect(
                    &mut frame,
                    rect.clone(),
                    debug_color.alpha(0.3),
                    &draw_parameters,
                );
            }

            instance
                .renderer()
                .draw(&mut frame, rect.clone(), &draw_parameters);

            if self.debug_mode {
                self.renderer.outline_rect(
                    &mut frame,
                    rect.clone(),
                    debug_color.alpha(0.8),
                    4.0,
                    &draw_parameters,
                );
            }
        }

        self.window.request_redraw();
        frame.finish().unwrap();
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.display.resize((width, height));
    }
}
