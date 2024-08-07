use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::error::Error;

use crate::texture::Texture;
use crate::types::*;

/// Single-textured font.
pub struct Font<'a> {
    texture: Texture<'a>,
    map: &'static str,
}

impl<'a> Font<'a> {
    pub fn load(texture: Texture<'a>, map: &'static str) -> Result<Self, Error> {
        Ok(Self { texture, map })
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, string: &str, position: Point, scale: Size) {
        let mut x = position.x;
        let mut y = position.y;
        string.chars().for_each(|char| {
            if char == '\n' {
                x = position.x;
                y += scale.h as i32;
            } else {
                let idx = self.map.find(char).unwrap_or(self.map.len());
                self.texture.draw_idx(
                    canvas,
                    Rect {
                        x,
                        y,
                        w: scale.w,
                        h: scale.h,
                    },
                    idx,
                );
                x += scale.w as i32;
            }
        })
    }
}
