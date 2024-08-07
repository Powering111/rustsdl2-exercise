use std::rc::Rc;

use crate::render::Canvas;

use crate::error::Error;

use crate::texture::Texture;
use crate::types::*;

pub type Font<'a> = Rc<FontInner<'a>>;

/// Single-textured font.
pub struct FontInner<'a> {
    texture: Texture<'a>,
    map: &'static str,
}

pub fn load_font<'a>(texture: Texture<'a>, map: &'static str) -> Result<Font<'a>, Error> {
    Ok(Rc::new(FontInner { texture, map }))
}

impl<'a> FontInner<'a> {
    pub fn draw(&self, canvas: Canvas, string: &str, position: Vec2, scale: Vec2) {
        let mut x = position.x;
        let mut y = position.y;
        string.chars().for_each(|char| {
            if char == '\n' {
                x = position.x;
                y += scale.y as i32;
            } else {
                let idx = self.map.find(char).unwrap_or(self.map.len());
                self.texture.draw_idx(
                    canvas,
                    Rect {
                        x,
                        y,
                        w: scale.x,
                        h: scale.y,
                    },
                    idx,
                );
                x += scale.x as i32;
            }
        })
    }
}
