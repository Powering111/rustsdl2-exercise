use sdl2::{render::TextureCreator, video::WindowContext};
use std::{cell::RefCell, collections::HashMap, path::Path};

use super::texture::{self, Texture};

/// Texture manager holding sdl2::render::texture_creator
/// Index texture by name(&'static str).
pub struct TextureManager<'a> {
    textures: HashMap<&'static str, Texture<'a>>,
}

impl<'a> TextureManager<'a> {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }

    pub fn load(
        &mut self,
        texture_creator: &'a TextureCreator<WindowContext>,
        name: &'static str,
        path: &Path,
    ) -> Result<(), crate::error::Error> {
        let new_texture = if path.extension().is_some_and(|ext| ext == "json") {
            texture::load_from_json(texture_creator, path)?
        } else {
            texture::load_from_file(texture_creator, path)?
        };
        self.textures.insert(name, new_texture);
        Ok(())
    }

    pub fn get(&self, name: &'static str) -> Texture<'a> {
        match self.textures.get(name) {
            Some(txt) => txt.clone(),
            None => self.textures.get("no_texture").unwrap().clone(),
        }
    }
}
