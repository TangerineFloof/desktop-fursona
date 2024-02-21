use std::time::Instant;

pub struct Time {
    prev: Instant,
}

impl Time {
    pub fn new() -> Self {
        Self {
            prev: Instant::now(),
        }
    }

    pub fn delta_ms(&mut self) -> u32 {
        let now = Instant::now();
        let delta = now.duration_since(self.prev).as_millis() as u32;
        self.prev = now;
        delta
    }
}
