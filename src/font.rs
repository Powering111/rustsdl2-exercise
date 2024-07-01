use std::path::Path;

use sdl2::render::{TextureCreator,Canvas};
use sdl2::video::{Window, WindowContext};

use crate::error::Error;

use crate::texture::{Texture, Rect, Point, Size};

pub struct Font<'a> {
    texture: Texture<'a>,
    map: &'static str,
}

impl<'a> Font<'a> {
    pub fn load(
        texture_creator: &'a TextureCreator<WindowContext>,
        path: &Path,
        map: &'static str,
    ) -> Result<Self, Error> {
        Ok(Self {
            texture: Texture::load_from_json(texture_creator, path)?,
            map,
        })
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, string: &str, position: Point, scale: Size) {
        let mut x = position.x;
        let y = position.y;
        string.chars().for_each(|char| {
                let idx = self.map.find(char).unwrap_or(self.map.len());
                self.texture.draw(canvas, Rect{ x, y, w: scale.w, h: scale.h }, idx);
                x += scale.w as i32;
            }
        )
    }
}
