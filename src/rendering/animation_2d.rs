use std::rc::Rc;

use glium::texture::CompressedTexture2d;

use super::{Animation, Renderer2D};

pub struct Keyframe2D {
    pub duration_ms: u32,
    pub image: Rc<CompressedTexture2d>,
}

#[derive(Copy, Clone)]
struct CurrentFrame {
    index: usize,
    time_remaining: u32,
}

enum AnimationState {
    NotStarted,
    Active(CurrentFrame),
    Finished,
}

pub struct Animation2D {
    keyframes: Vec<Keyframe2D>,
    state: AnimationState,
}

impl Animation2D {
    pub fn new(keyframes: Vec<Keyframe2D>) -> Self {
        Self {
            keyframes,
            state: AnimationState::NotStarted,
        }
    }

    fn get_fresh_frame(&self, index: usize) -> Option<CurrentFrame> {
        if let Some(keyframe) = self.keyframes.get(index) {
            Some(CurrentFrame {
                index,
                time_remaining: keyframe.duration_ms,
            })
        } else {
            None
        }
    }
}

impl Animation for Animation2D {
    type ValidRenderer = Renderer2D;

    fn advance(&mut self, delta_t_ms: u32, renderer: &mut Self::ValidRenderer) -> () {
        let (current_frame, delta_t_ms) = match self.state {
            AnimationState::NotStarted => {
                // We'll set the delta_t_ms to 0 here, because we're starting the
                // animation -- there isn't a delta from the previous frame, because
                // there wasn't a previous frame.
                (self.get_fresh_frame(0), 0)
            }
            AnimationState::Active(current) => (Some(current), delta_t_ms),
            AnimationState::Finished => (None, 0),
        };

        // If we have no current frame, we can't do anything. We should return
        // here, AND make sure that our state is set to finished
        let mut current_frame = match current_frame {
            Some(f) => f,
            None => {
                self.state = AnimationState::Finished;
                return;
            }
        };

        // If the current frame would end, move to the next one
        let mut delta_t_ms = delta_t_ms;
        while delta_t_ms > current_frame.time_remaining {
            delta_t_ms -= current_frame.time_remaining;
            let next = self.get_fresh_frame(current_frame.index + 1);
            current_frame = match next {
                Some(f) => f,
                None => {
                    self.state = AnimationState::Finished;
                    return;
                }
            };
        }

        // We now have THE current and correct frame. If this is different from
        // the one that we have in state, then update our state and configure
        // the renderer
        let should_update_state = match self.state {
            AnimationState::NotStarted | AnimationState::Finished => true,
            AnimationState::Active(c) => c.index != current_frame.index,
        };

        if should_update_state {
            self.state = AnimationState::Active(current_frame);
            renderer.set_texture(self.keyframes[current_frame.index].image.clone());
        }

        // Decrease the remaining time on this frame, but otherwise no other
        // action needed
        if delta_t_ms > 0 {
            match &mut self.state {
                AnimationState::Active(c) => {
                    c.time_remaining -= delta_t_ms;
                }
                _ => (),
            }
        }
    }

    fn is_finished(&self) -> bool {
        match self.state {
            AnimationState::Finished => true,
            _ => false,
        }
    }

    fn reset(&mut self) {
        self.state = AnimationState::NotStarted;
    }
}
