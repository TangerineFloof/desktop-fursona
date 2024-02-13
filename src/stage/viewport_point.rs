use std::ops::Add;

// A point, measured in pixels, that is relative to the viewport. These would
// be pixel coordinates as experienced by the user.
pub struct ViewportPoint {
    pub x: u32,
    pub y: u32,
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

impl Add<(u32, u32)> for ViewportPoint {
    type Output = ViewportPoint;

    fn add(self, other: (u32, u32)) -> ViewportPoint {
        ViewportPoint {
            x: self.x + other.0,
            y: self.y + other.1,
        }
    }
}
