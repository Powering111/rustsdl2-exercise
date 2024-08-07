use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::types::*;

use crate::game::entity::Entity;
use crate::game::ui::UIElement;

struct Camera {
    center: Point,
    scale: u32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            center: Point { x: 0, y: 0 },
            scale: 1,
        }
    }
}

pub struct Scene<'a> {
    camera: Camera,
    ui: Vec<Box<dyn UIElement>>,
    entity: Vec<Box<dyn Entity + 'a>>,
}

impl<'a> Scene<'a> {
    pub fn new() -> Self {
        Self {
            camera: Camera::default(),
            ui: Vec::new(),
            entity: Vec::new(),
        }
    }
    pub fn add_entity(&mut self, entity: impl Entity + 'a) {
        self.entity.push(Box::new(entity));
    }

    pub fn update(&mut self) {
        for entity in self.entity.iter_mut() {
            entity.update();
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        for entity in self.entity.iter() {
            entity.draw(canvas);
        }
        for ui in self.ui.iter() {
            ui.draw(canvas);
        }
    }
}
