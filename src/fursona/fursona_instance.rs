use super::Fursona;
use crate::{
    behaviors::{pace::PaceBehavior, AnimationDictionary, Behavior, BehaviorContext},
    rendering::{Animation2D, Keyframe2D, Renderer, Renderer2D, TextureCache},
    stage::{Stage, ViewportPoint, ViewportRect},
};

enum FursonaInstanceRendering {
    TwoD {
        behavior: Box<dyn Behavior<Renderer2D>>,
        renderer: Renderer2D,
    },
}

pub struct FursonaInstance {
    position: ViewportPoint,
    width: f32,
    height: f32,
    rendering: FursonaInstanceRendering,
}

impl FursonaInstance {
    pub fn new(_fursona: &Fursona, stage: &Stage) -> Self {
        let mut texture_cache = TextureCache::new(stage);

        let right = texture_cache.get("./jack_by_nal_cinnamonspots.png");
        let left = texture_cache.get("./jack_by_nal_cinnamonspots_flipped.png");
        let mut anim_dictionary = AnimationDictionary {
            walk_left: Some(Box::new(move || {
                Box::new(Animation2D::new(vec![Keyframe2D {
                    duration_ms: 2000,
                    image: right.clone(),
                }]))
            })),
            walk_right: Some(Box::new(move || {
                Box::new(Animation2D::new(vec![Keyframe2D {
                    duration_ms: 2000,
                    image: left.clone(),
                }]))
            })),
        };

        FursonaInstance {
            position: ViewportPoint {
                x: stage.viewport.left() as f32,
                y: stage.viewport.top() as f32,
            },
            width: 0.0,  // TODO
            height: 0.0, // TODO
            rendering: FursonaInstanceRendering::TwoD {
                behavior: Box::new(PaceBehavior::new(&mut anim_dictionary).unwrap()),
                renderer: Renderer2D::new(&stage),
            },
        }
    }

    pub fn bounding_box(&self) -> ViewportRect {
        ViewportRect {
            x: self.position.x,
            y: self.position.y,
            width: self.width,
            height: self.height,
        }
    }

    pub fn renderer(&self) -> &dyn Renderer {
        match &self.rendering {
            FursonaInstanceRendering::TwoD { renderer, .. } => renderer,
        }
    }

    pub fn update<'a>(&mut self, delta_t_ms: u32, stage: &'a Stage) -> () {
        // Process the current behavior
        let behavior_context = BehaviorContext {
            position: self.position.clone(),
            stage,
        };
        let result = match &mut self.rendering {
            FursonaInstanceRendering::TwoD { behavior, renderer } => {
                behavior.advance(delta_t_ms, renderer, behavior_context)
            }
        };

        self.position = result.bounding_box.position();
        self.width = result.bounding_box.width;
        self.height = result.bounding_box.height;
    }
}
