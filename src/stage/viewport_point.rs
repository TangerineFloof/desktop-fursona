use std::ops::Add;

// A point, measured in pixels, that is relative to the viewport. These would
// be pixel coordinates as experienced by the user.
pub struct ViewportPoint {
    pub x: f32,
    pub y: f32,
}

impl Add<ViewportPoint> for ViewportPoint {
    type Output = ViewportPoint;

    fn add(self, other: ViewportPoint) -> ViewportPoint {
        ViewportPoint {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
