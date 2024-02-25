use super::ViewportPoint;

// A point, measured in pixels, that is relative to the viewport. These would
// be pixel coordinates as experienced by the user.
pub struct ViewportRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl ViewportRect {
    pub fn position(&self) -> ViewportPoint {
        ViewportPoint {
            x: self.x,
            y: self.y,
        }
    }
}
