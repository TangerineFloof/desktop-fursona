mod renderer;
mod texture_cache;

pub use renderer::Renderer;
pub use texture_cache::TextureCache;

pub struct RendererCoord {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone)]
pub struct RendererRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub pixel_width: f32,
    pub pixel_height: f32,
}

impl RendererRect {
    pub fn scale(&mut self, scalar: f32) -> () {
        let next_width = self.width * scalar;
        let diff_width = next_width - self.width;
        let next_x = self.x + diff_width / 2.0f32;

        let next_height = self.height * scalar;
        let diff_height = next_height - self.height;
        let next_y = self.y + diff_height / 2.0f32;

        self.x = next_x;
        self.y = next_y;
        self.width = next_width;
        self.height = next_height;
        self.pixel_width *= scalar;
        self.pixel_height *= scalar;
    }
}

#[derive(Clone)]
pub struct Color(pub f32, pub f32, pub f32, pub f32);

impl Color {
    pub fn alpha(&self, alpha: f32) -> Self {
        Self(self.0, self.1, self.2, alpha)
    }
}
