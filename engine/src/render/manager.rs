use sdl2::{render::TextureCreator, video::WindowContext};
use std::{collections::HashMap, path::Path};

use crate::error::Error;
use crate::render::texture::{self, Texture};

/// Texture manager holding sdl2::render::texture_creator
/// Index texture by name(&'static str).
pub struct TextureManager {
    texture_creator: TextureCreator<WindowContext>,
    textures: HashMap<&'static str, Texture>,
}

impl TextureManager {
    pub fn new(texture_creator: TextureCreator<WindowContext>) -> Self {
        Self {
            texture_creator,
            textures: HashMap::new(),
        }
    }

    pub fn load(&mut self, name: &'static str, path: &Path) -> Result<(), Error> {
        let new_texture = if path.extension().is_some_and(|ext| ext == "json") {
            texture::load_from_json(&self.texture_creator, path)?
        } else {
            texture::load_from_file(&self.texture_creator, path)?
        }
        .clone();

        if self.textures.contains_key(name) {
            Err(Error::AlreadyExists)
        } else {
            self.textures.insert(name, new_texture);
            println!("loaded texture {name:}");
            Ok(())
        }
    }

    pub fn get(&self, name: &'static str) -> Texture {
        match self.textures.get(name) {
            Some(txt) => txt.clone(),
            None => self
                .textures
                .get("no_texture")
                .expect(format!("no texture named {name:}").as_str())
                .clone(),
        }
    }
}
