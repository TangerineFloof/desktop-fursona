mod renderer_2d;
mod renderer_coord;

use glium::Frame;

pub use renderer_2d::Renderer2D;
pub use renderer_coord::RendererCoord;

pub trait Renderer {
    fn draw(&self, frame: &mut Frame) -> ();
}
