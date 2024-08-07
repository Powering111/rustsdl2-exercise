use crate::render::manager::TextureManager;
use crate::render::texture::Texture;
use crate::types::*;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub trait Entity {
    fn update(&mut self);
    fn draw(&self, canvas: &mut Canvas<Window>);
}

/// Human entity for test
pub struct HumanEntity<'a> {
    texture: Texture<'a>,
    position: Point,
    anim_idx: usize,
    anim_delay: usize,
}

impl<'a> HumanEntity<'a> {
    pub fn new(texture_manager: &'a TextureManager) -> Self {
        Self {
            texture: texture_manager.get("sprite.human"),
            position: Point { x: 300, y: 350 },
            anim_idx: 0,
            anim_delay: 20,
        }
    }
}

impl<'a> Entity for HumanEntity<'a> {
    fn update(&mut self) {
        self.anim_delay -= 1;
        if self.anim_delay == 0 {
            // next frame
            self.anim_delay = 20;
            self.anim_idx = (self.anim_idx + 1) % self.texture.len();
        }
    }

    fn draw(&self, canvas: &mut Canvas<Window>) {
        self.texture.draw_idx(
            canvas,
            Rect::from_center_size(self.position, Size { w: 200, h: 200 }),
            self.anim_idx,
        );
    }
}
