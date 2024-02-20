use super::Fursona;
use crate::{
    rendering::{Renderer, Renderer2D},
    stage::{Stage, ViewportPoint},
};

pub struct FursonaInstance {
    pub position: ViewportPoint,
    pub renderer: Box<dyn Renderer>,
}

impl FursonaInstance {
    pub fn new(_fursona: &Fursona, stage: &Stage) -> Self {
        FursonaInstance {
            position: stage.viewport.top_left(),
            renderer: Box::new(Renderer2D::new(&stage, "./jack_by_nal_cinnamonspots.png")),
        }
    }
}
