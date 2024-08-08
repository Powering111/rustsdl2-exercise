use crate::game::entity::Entity;
use crate::game::ui::UIElement;
use crate::render::Renderer;
use crate::types::*;

/// Scene information used to determine what to draw
pub struct SceneInfo {
    pub camera: Camera,
}

#[derive(Clone, Copy)]
pub struct Camera {
    /// world space coordinate of the camera.
    center: Vec2,

    /// zoom level of the camera.
    ///
    /// `world : view = 1: *scale*`
    ///
    /// which means 1 unit in world space become *scale* pixels on screen.
    zoom: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            center: Vec2 { x: 0, y: 0 },
            zoom: 0.5,
        }
    }
}

impl Camera {
    /// transforms *rect* to the view coordinate
    pub fn transform(&self, rect: Rect) -> Rect {
        let object_center = rect.point_center();
        let transformed_center = (object_center - self.center) * self.zoom;
        let transformed_size = rect.size() * self.zoom;
        Rect::from_center_size(transformed_center, transformed_size)
    }
}

pub struct Scene {
    scene_info: SceneInfo,
    ui: Vec<Box<dyn UIElement>>,
    entity: Vec<Box<dyn Entity>>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            scene_info: SceneInfo {
                camera: Camera::default(),
            },
            ui: Vec::new(),
            entity: Vec::new(),
        }
    }
    pub fn add_entity(&mut self, entity: Box<dyn Entity>) {
        self.entity.push(entity);
    }

    pub fn update(&mut self) {
        for entity in self.entity.iter_mut() {
            entity.update();
        }
    }

    pub fn render(&self, renderer: &mut Renderer) {
        for entity in self.entity.iter() {
            entity.draw(renderer, &self.scene_info);
        }
        for ui in self.ui.iter() {
            ui.draw(renderer, &self.scene_info);
        }

        // Debug: crosshair at the center
        let center = Vec2 {
            x: renderer.render_info.screen_size.x / 2,
            y: renderer.render_info.screen_size.y / 2,
        };
        renderer
            .canvas
            .set_draw_color(sdl2::pixels::Color::RGB(255, 0, 255));
        renderer
            .canvas
            .draw_line(
                Vec2 {
                    x: center.x - 10,
                    y: center.y,
                },
                Vec2 {
                    x: center.x + 10,
                    y: center.y,
                },
            )
            .unwrap();
        renderer
            .canvas
            .draw_line(
                Vec2 {
                    x: center.x,
                    y: center.y - 10,
                },
                Vec2 {
                    x: center.x,
                    y: center.y + 10,
                },
            )
            .unwrap();
    }

    // for debug
    pub fn get_position(&self) -> Vec2 {
        self.scene_info.camera.center
    }
    pub fn set_position(&mut self, pos: Vec2) {
        self.scene_info.camera.center = pos;
    }
    pub fn add_zoom(&mut self, zoom: f32) {
        self.scene_info.camera.zoom *= 1.05f32.powf(zoom);
        if self.scene_info.camera.zoom < 0.1 {
            self.scene_info.camera.zoom = 0.1;
        }
        if self.scene_info.camera.zoom > 1.0 {
            self.scene_info.camera.zoom = 1.0;
        }
    }
}
