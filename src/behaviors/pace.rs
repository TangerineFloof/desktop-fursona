use super::{AnimationDictionary, Behavior, BehaviorContext, BehaviorPreview, BehaviorResult};
use crate::{
    rendering::{Animation, Renderer},
    stage::ViewportRect,
};

const SPEED_PIXELS_PER_SECOND: u32 = 240;
const MILLISECONDS_PER_PIXEL: f32 = (SPEED_PIXELS_PER_SECOND as f32) / 1000.0;

#[derive(Debug, PartialEq)]
enum PaceDirection {
    Left,
    Right,
}

pub struct PaceBehavior<T: Renderer> {
    direction: PaceDirection,
    walk_left: Box<dyn Animation<ValidRenderer = T>>,
    walk_right: Box<dyn Animation<ValidRenderer = T>>,
}

impl<T: Renderer> BehaviorPreview<T> for PaceBehavior<T> {
    fn is_possible(anims: &AnimationDictionary<T>) -> bool {
        anims.walk_left.is_some() && anims.walk_right.is_some()
    }
}

impl<T: Renderer> Behavior<T> for PaceBehavior<T> {
    fn new(anims: &mut AnimationDictionary<T>) -> Option<Self>
    where
        Self: Sized,
    {
        let walk_left = if let Some(anim) = &mut anims.walk_left {
            anim()
        } else {
            return None;
        };

        let walk_right = if let Some(anim) = &mut anims.walk_right {
            anim()
        } else {
            return None;
        };

        Some(Self {
            direction: PaceDirection::Right,
            walk_left,
            walk_right,
        })
    }

    fn advance(
        &mut self,
        delta_t_ms: u32,
        renderer: &mut T,
        context: BehaviorContext,
    ) -> BehaviorResult {
        // Determine our boundaries
        let left = context.stage.viewport.left();
        let anim = match self.direction {
            PaceDirection::Left => &self.walk_left,
            PaceDirection::Right => &self.walk_right,
        };
        let right = context.stage.viewport.right() - anim.intrinsic_dimensions().0;

        // Determine what our new x position should be
        let dist = (delta_t_ms as f32) * MILLISECONDS_PER_PIXEL;
        let x = match self.direction {
            PaceDirection::Left => context.position.x - dist,
            PaceDirection::Right => context.position.x + dist,
        };
        let x = x.clamp(left, right);

        // If we've reached the edge, then we'll swap directions
        let updated_direction = match self.direction {
            PaceDirection::Left => {
                if x <= left {
                    PaceDirection::Right
                } else {
                    PaceDirection::Left
                }
            }
            PaceDirection::Right => {
                if x >= right {
                    PaceDirection::Left
                } else {
                    PaceDirection::Right
                }
            }
        };

        let did_change = updated_direction != self.direction;
        if did_change {
            println!(
                "[pace] changing {:?} to {updated_direction:?}",
                self.direction
            );
            self.direction = updated_direction;
        }

        // Update our animation
        let anim = match self.direction {
            PaceDirection::Left => &mut self.walk_left,
            PaceDirection::Right => &mut self.walk_right,
        };

        if did_change {
            anim.reset();
        }

        anim.advance(delta_t_ms, renderer);

        // Return the result
        let (width, height) = anim.intrinsic_dimensions();
        BehaviorResult {
            bounding_box: ViewportRect {
                x,
                y: context.position.y,
                width,
                height,
            },
        }
    }
}
