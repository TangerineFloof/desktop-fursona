use super::{viewport_point::ViewportPoint, viewport_rect::ViewportRect};
use crate::rendering::{RendererCoord, RendererRect};
use std::rc::Rc;
use winit::{dpi::LogicalSize, window::Window};

// TODO: This isn't consistent/safe to interact with prior to the Resumed event.
// How do we want to express this, once I start handling errors correctly?
pub struct Viewport {
    window: Rc<Window>,
}

impl Viewport {
    pub fn new(window: Rc<Window>) -> Viewport {
        Self { window }
    }

    pub fn convert_point_to_renderer_coord(&self, point: ViewportPoint) -> RendererCoord {
        let scale_factor = self.window.scale_factor();
        let size: LogicalSize<f32> = self.window.inner_size().to_logical(scale_factor);
        let half_width = size.width / 2.0;
        let half_height = size.height / 2.0;

        RendererCoord {
            x: (point.x as f32 - half_width) / half_width,
            y: (half_height - point.y as f32) / half_height,
        }
    }

    pub fn convert_rect(&self, rect: ViewportRect) -> RendererRect {
        let scale_factor = self.window.scale_factor();
        let size: LogicalSize<f32> = self.window.inner_size().to_logical(scale_factor);
        let half_width = size.width / 2.0;
        let half_height = size.height / 2.0;

        RendererRect {
            x: (rect.x as f32 - half_width) / half_width,
            y: (half_height - rect.y as f32) / half_height,
            width: (rect.width as f32) / size.width,
            height: (rect.height as f32) / size.height,
        }
    }
}

#[cfg(target_os = "macos")]
fn get_menu_bar_height() -> f64 {
    use icrate::AppKit::NSApplication;
    use icrate::Foundation::MainThreadMarker;

    let mtm = MainThreadMarker::new().expect("must be on the main thread");
    let app = NSApplication::sharedApplication(mtm);
    let menu = unsafe { app.mainMenu().unwrap() };
    let height = unsafe { menu.menuBarHeight() };
    height
}

#[cfg(target_os = "macos")]
impl Viewport {
    pub fn left(&self) -> f32 {
        0.0
    }

    pub fn right(&self) -> f32 {
        let scale_factor = self.window.scale_factor();
        self.window.inner_size().to_logical(scale_factor).width
    }

    pub fn top(&self) -> f32 {
        get_menu_bar_height() as f32
    }
}
