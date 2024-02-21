use std::rc::Rc;

use glium::texture::CompressedTexture2d;
use std::collections::HashMap;
use std::path::Path;

use crate::stage::Stage;

pub struct TextureCache<'a> {
    cache: HashMap<&'static str, Rc<CompressedTexture2d>>,
    stage: &'a Stage,
}

impl<'a> TextureCache<'a> {
    pub fn new(stage: &'a Stage) -> Self {
        Self {
            cache: HashMap::new(),
            stage,
        }
    }

    pub fn get(&mut self, filename: &'static str) -> Rc<CompressedTexture2d> {
        // If it's cached, return the reference
        if let Some(cached) = self.cache.get(filename) {
            return cached.clone();
        }

        // It isn't cached, so we need to create it
        let created = Rc::new(self.load(filename));
        self.cache.insert(filename, created.clone());
        created
    }

    fn load(&self, filename: &str) -> CompressedTexture2d {
        let image = image::open(Path::new(filename)).unwrap().to_rgba8();
        let image_dimensions = image.dimensions();
        let image =
            glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        CompressedTexture2d::new(&self.stage.display, image).unwrap()
    }
}
