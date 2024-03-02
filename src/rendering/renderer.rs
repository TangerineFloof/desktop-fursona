mod square_renderer;

use glium::{Display, DrawParameters, Frame};
use glutin::surface::WindowSurface;
use square_renderer::SquareRenderer;

use super::{Color, RendererRect};

pub struct Renderer {
    square_renderer: SquareRenderer,
}

impl Renderer {
    pub fn new(display: &Display<WindowSurface>) -> Self {
        Self {
            square_renderer: SquareRenderer::new(display),
        }
    }

    pub fn fill_rect(
        &self,
        frame: &mut Frame,
        rect: RendererRect,
        color: Color,
        base_draw_parameters: &DrawParameters,
    ) -> () {
        self.square_renderer
            .draw(frame, rect, &color, (1.0, 1.0), base_draw_parameters);
    }

    pub fn outline_rect(
        &self,
        frame: &mut Frame,
        rect: RendererRect,
        color: Color,
        thickness_pixels: f32,
        base_draw_parameters: &DrawParameters,
    ) -> () {
        let thickness = (
            thickness_pixels / rect.pixel_width,
            thickness_pixels / rect.pixel_height,
        );
        self.square_renderer
            .draw(frame, rect, &color, thickness, base_draw_parameters);
    }
}
