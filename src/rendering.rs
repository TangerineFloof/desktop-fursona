mod animation_2d;
mod renderer_2d;
mod renderer_coord;
mod texture_cache;

use crate::stage::Viewport;
use glium::Frame;

pub use animation_2d::{Animation2D, Keyframe2D};
pub use renderer_2d::Renderer2D;
pub use renderer_coord::RendererCoord;
pub use texture_cache::TextureCache;

pub trait Renderer {
    fn draw(
        &self,
        frame: &mut Frame,
        viewport: &Viewport,
        position: RendererCoord,
        scale: (f32, f32),
    ) -> ();
}

pub trait Animation {
    type ValidRenderer;

    fn advance(&mut self, delta_t_ms: u32, renderer: &mut Self::ValidRenderer) -> ();
    fn is_finished(&self) -> bool;
    fn reset(&mut self) -> ();
}
