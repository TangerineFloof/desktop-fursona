mod texture_cache;

pub use texture_cache::TextureCache;

pub struct RendererCoord {
    pub x: f32,
    pub y: f32,
}

pub struct RendererRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}
