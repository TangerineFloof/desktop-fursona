pub mod animation_2d;

pub trait Animation {
    type ValidRenderer;

    fn advance(&mut self, delta_t_ms: u32, renderer: &mut Self::ValidRenderer) -> ();
    fn is_finished(&self) -> bool;
    fn intrinsic_dimensions(&self) -> (f32, f32);
    fn reset(&mut self) -> ();
}

pub type AnimationConstructor<T> = dyn FnMut() -> Box<dyn Animation<ValidRenderer = T>>;
