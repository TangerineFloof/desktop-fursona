pub mod renderer_2d;

use crate::rendering::RendererRect;
use glium::Frame;

pub trait FursonaRenderer {
    fn draw(&self, frame: &mut Frame, rect: RendererRect) -> ();
}
