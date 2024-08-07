use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::render::RenderInfo;
use crate::types::*;

use crate::game::entity::Entity;
use crate::game::ui::UIElement;

/// Scene information used to determine what to draw
pub struct SceneInfo {
    pub camera: Camera,
}

#[derive(Clone, Copy)]
pub struct Camera {
    /// world space coordinate of the camera.
    center: Point,

    /// 1000 * zoom level of the camera.
    ///
    /// `world : view = 1000 : *scale*`
    ///
    /// which means 1 unit in world space become *scale*/1000 pixels on screen.
    zoom: i32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            center: Point { x: 0, y: 0 },
            zoom: 1000,
        }
    }
}

impl Camera {
    /// transforms *rect* to the view coordinate
    pub fn transform(&self, rect: Rect) -> Rect {
        let transform_offset: Size = self.center.into();
        rect.transform((-transform_offset * self.zoom) / 1000)
            .scale_up(self.zoom)
            .scale_down(1000)
    }
}

pub struct Scene<'a> {
    scene_info: SceneInfo,
    ui: Vec<Box<dyn UIElement>>,
    entity: Vec<Box<dyn Entity + 'a>>,
}

impl<'a> Scene<'a> {
    pub fn new() -> Self {
        Self {
            scene_info: SceneInfo {
                camera: Camera::default(),
            },
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

    pub fn render(&self, canvas: &mut Canvas<Window>, render_info: &RenderInfo) {
        for entity in self.entity.iter() {
            entity.draw(canvas, render_info, &self.scene_info);
        }
        for ui in self.ui.iter() {
            ui.draw(canvas, render_info, &self.scene_info);
        }
    }

    // for debug
    pub fn get_position(&self) -> Point {
        self.scene_info.camera.center
    }
    pub fn set_position(&mut self, pos: Point) {
        self.scene_info.camera.center = pos;
    }
    pub fn add_zoom(&mut self, zoom: i32) {
        self.scene_info.camera.zoom += zoom * 100;
        if self.scene_info.camera.zoom < 100 {
            self.scene_info.camera.zoom = 100;
        }
    }
}
