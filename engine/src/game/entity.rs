use std::cell::RefCell;
use std::rc::Rc;

use crate::render::texture::Texture;
use crate::types::*;

pub type Entity = Rc<RefCell<dyn EntityTrait>>;
pub struct EntityDrawInfo {
    pub world_rect: Rect,
    pub texture: Texture,
    pub texture_idx: usize,
}

pub trait EntityTrait {
    fn pos(&self) -> Vec2;
    fn set_pos(&mut self, new_pos: Vec2);

    fn update(&mut self);
    fn get_draw_info(&self) -> EntityDrawInfo;
}

/// Human entity for test
pub struct CharacterEntity {
    texture: Texture,
    position: Vec2,
    anim_idx: usize,
    anim_delay: usize,
}

impl CharacterEntity {
    pub fn new(texture: Texture, pos: Vec2) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            texture,
            position: pos,
            anim_idx: 0,
            anim_delay: 20,
        }))
    }
}

impl EntityTrait for CharacterEntity {
    fn pos(&self) -> Vec2 {
        self.position
    }
    fn set_pos(&mut self, new_pos: Vec2) {
        self.position = new_pos;
    }

    fn update(&mut self) {
        self.anim_delay -= 1;
        if self.anim_delay == 0 {
            // next frame
            self.anim_delay = 20;
            self.anim_idx = (self.anim_idx + 1) % self.texture.len();
        }
    }
    fn get_draw_info(&self) -> EntityDrawInfo {
        EntityDrawInfo {
            world_rect: Rect::from_center_size(self.position, Vec2 { x: 200, y: 200 }),
            texture: self.texture.clone(),
            texture_idx: self.anim_idx,
        }
    }
}
