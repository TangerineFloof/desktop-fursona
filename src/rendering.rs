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

#[derive(Clone)]
pub struct Color(pub f32, pub f32, pub f32, pub f32);

impl Color {
    pub fn alpha(&self, alpha: f32) -> Self {
        Self(self.0, self.1, self.2, alpha)
    }
}
