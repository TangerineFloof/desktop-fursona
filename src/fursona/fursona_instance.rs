use super::Fursona;
use crate::{
    rendering::{Animation, Animation2D, Keyframe2D, Renderer, Renderer2D, TextureCache},
    stage::{Stage, ViewportPoint, ViewportRect},
};

enum FursonaInstanceRendering {
    TwoD {
        animation: Animation2D,
        renderer: Renderer2D,
    },
}

pub struct FursonaInstance {
    position: ViewportPoint,
    rendering: FursonaInstanceRendering,
}

impl FursonaInstance {
    pub fn new(_fursona: &Fursona, stage: &Stage) -> Self {
        let mut texture_cache = TextureCache::new(stage);

        FursonaInstance {
            position: stage.viewport.top_left(),
            rendering: FursonaInstanceRendering::TwoD {
                animation: Animation2D::new(vec![
                    Keyframe2D {
                        duration_ms: 2000,
                        image: texture_cache.get("./jack_by_nal_cinnamonspots.png"),
                    },
                    Keyframe2D {
                        duration_ms: 2000,
                        image: texture_cache.get("./jack_by_nal_cinnamonspots_flipped.png"),
                    },
                ]),
                renderer: Renderer2D::new(&stage),
            },
        }
    }

    pub fn bounding_box(&self) -> ViewportRect {
        let (intrinsic_width, intrinsic_height) = match &self.rendering {
            FursonaInstanceRendering::TwoD { animation, .. } => {
                animation.image_intrinsic_dimensions()
            }
        };

        ViewportRect {
            x: self.position.x,
            y: self.position.y,
            width: intrinsic_width,
            height: intrinsic_height,
        }
    }

    pub fn renderer(&self) -> &dyn Renderer {
        match &self.rendering {
            FursonaInstanceRendering::TwoD { renderer, .. } => renderer,
        }
    }

    pub fn update(&mut self, delta_t_ms: u32) -> () {
        // Update the animation
        match &mut self.rendering {
            FursonaInstanceRendering::TwoD {
                animation,
                renderer,
            } => animation.advance(delta_t_ms, renderer),
        };

        // TEMP: If the animation has ended, reset it
        match &mut self.rendering {
            FursonaInstanceRendering::TwoD { animation, .. } => {
                if animation.is_finished() {
                    println!("resetting animation");
                    animation.reset();
                }
            }
        }
    }
}
