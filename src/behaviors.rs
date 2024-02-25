pub mod pace;

use crate::{
    rendering::{AnimationConstructor, Renderer},
    stage::{Stage, ViewportPoint, ViewportRect},
};

type AnimDictionaryEntry<T> = Box<AnimationConstructor<T>>;

pub struct AnimationDictionary<T>
where
    T: Renderer,
{
    pub walk_right: Option<AnimDictionaryEntry<T>>,
    pub walk_left: Option<AnimDictionaryEntry<T>>,
}

pub struct BehaviorContext<'a> {
    pub position: ViewportPoint,
    pub stage: &'a Stage,
}

pub struct BehaviorResult {
    pub bounding_box: ViewportRect,
}

pub trait Behavior<T: Renderer> {
    fn new(anims: &mut AnimationDictionary<T>) -> Option<Self>
    where
        Self: Sized;

    fn advance(
        &mut self,
        delta_t_ms: u32,
        renderer: &mut T,
        context: BehaviorContext,
    ) -> BehaviorResult;
}

pub trait BehaviorPreview<T: Renderer> {
    // Given a set of the animations available, determines whether this
    // behavior is possible to run or not.
    // This isn't REQUIRED to run, but this will be a predictor of whether
    // `Behavior::new` will return None or Some.
    fn is_possible(anims: &AnimationDictionary<T>) -> bool;
}
