use crate::render::manager::TextureManager;
use crate::render::texture::Texture;
use crate::render::Canvas;
use crate::types::*;

use crate::game::scene::SceneInfo;
use crate::render::RenderInfo;

pub trait Entity {
    fn update(&mut self);
    fn draw(&self, canvas: Canvas, render_info: &RenderInfo, scene_info: &SceneInfo);
}

/// Human entity for test
pub struct HumanEntity<'a> {
    texture: Texture<'a>,
    position: Point,
    anim_idx: usize,
    anim_delay: usize,
}

impl<'a> HumanEntity<'a> {
    pub fn new(texture_manager: &'a TextureManager, sprite_name: &'static str, pos: Point) -> Self {
        Self {
            texture: texture_manager.get(sprite_name),
            position: pos,
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

    fn draw(&self, canvas: Canvas, render_info: &RenderInfo, scene_info: &SceneInfo) {
        let world_rect = Rect::from_center_size(self.position, Size { w: 200, h: 200 });
        let view_rect = scene_info.camera.transform(world_rect);

        match crate::render::clip(view_rect, render_info.screen_size) {
            Some(screen_rect) => {
                self.texture.draw_idx(canvas, screen_rect, self.anim_idx);
            }
            None => (),
        }
    }
}
