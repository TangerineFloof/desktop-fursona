mod renderer_2d;
use glium::Frame;

pub use renderer_2d::Renderer2D;

pub trait Renderer {
    fn draw(&self, frame: &mut Frame) -> ();
}
