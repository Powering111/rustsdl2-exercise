use crate::render::manager::TextureManager;
use crate::render::texture::Texture;
use crate::render::Canvas;
use crate::{types::*, Renderer};

use crate::game::scene::SceneInfo;
use crate::render::RenderInfo;

pub trait Entity {
    fn update(&mut self);
    fn draw(&self, renderer: &mut Renderer, scene_info: &SceneInfo);
}

/// Human entity for test
pub struct HumanEntity {
    texture: Texture,
    position: Vec2,
    anim_idx: usize,
    anim_delay: usize,
}

impl HumanEntity {
    pub fn new(texture: Texture, pos: Vec2) -> Self {
        Self {
            texture,
            position: pos,
            anim_idx: 0,
            anim_delay: 20,
        }
    }
}

impl Entity for HumanEntity {
    fn update(&mut self) {
        self.anim_delay -= 1;
        if self.anim_delay == 0 {
            // next frame
            self.anim_delay = 20;
            self.anim_idx = (self.anim_idx + 1) % self.texture.len();
        }
    }

    fn draw(&self, renderer: &mut Renderer, scene_info: &SceneInfo) {
        let world_rect = Rect::from_center_size(self.position, Vec2 { x: 200, y: 200 });
        let view_rect = scene_info.camera.transform(world_rect);

        match crate::render::clip(view_rect, renderer.render_info.screen_size) {
            Some(screen_rect) => {
                self.texture.draw_idx(&mut renderer.canvas, screen_rect, self.anim_idx);
            }
            None => (),
        }
    }
}
