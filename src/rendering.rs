mod animation_2d;
mod renderer_2d;
mod texture_cache;

use glium::Frame;

pub use animation_2d::{Animation2D, Keyframe2D};
pub use renderer_2d::Renderer2D;
pub use texture_cache::TextureCache;

pub struct RendererCoord {
    pub x: f32,
    pub y: f32,
}

pub struct RendererRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

pub trait Renderer {
    fn draw(&self, frame: &mut Frame, rect: RendererRect) -> ();
}

pub trait Animation {
    type ValidRenderer;

    fn advance(&mut self, delta_t_ms: u32, renderer: &mut Self::ValidRenderer) -> ();
    fn is_finished(&self) -> bool;
    fn reset(&mut self) -> ();
}
