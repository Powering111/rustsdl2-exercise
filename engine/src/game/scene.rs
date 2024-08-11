use crate::game::entity::Entity;
use crate::game::ui::UIElement;
use crate::render::Renderer;
use crate::types::*;

use crate::game::entity::EntityDrawInfo;

/// Scene information used to determine what to draw
pub struct SceneInfo {
    pub camera: Camera,
}

pub struct Camera {
    /// world space coordinate of the camera.
    center: Vec2,
    attachment: Option<Entity>,

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
            attachment: None,

            zoom: 0.5,
        }
    }
}

impl Camera {
    /// transforms *rect* to the view coordinate
    pub fn transform(&self, rect: Rect) -> Rect {
        let camera_pos: Vec2 = if let Some(entity) = &self.attachment {
            entity.borrow().pos()
        } else {
            self.center
        };

        let object_center = rect.point_center();
        let transformed_center = (object_center - camera_pos) * self.zoom;
        let transformed_size = rect.size() * self.zoom;
        Rect::from_center_size(transformed_center, transformed_size)
    }

    pub fn attach(&mut self, target: Entity) {
        self.attachment = Some(target);
    }
    pub fn detach(&mut self) {
        self.attachment = None;
    }
}

/// Scene contains entity + background + UI.
/// There can be only one active scene at a time.
pub struct Scene {
    scene_info: SceneInfo,
    ui: Vec<Box<dyn UIElement>>,
    pub entity_list: Vec<Entity>,
    // TODO: background tile
}

impl Scene {
    pub fn new() -> Self {
        Self {
            scene_info: SceneInfo {
                camera: Camera::default(),
            },
            ui: Vec::new(),
            entity_list: Vec::new(),
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entity_list.push(entity);
    }

    pub fn add_ui(&mut self, ui: Box<dyn UIElement>) {
        self.ui.push(ui);
    }

    pub fn update(&mut self) {
        for entity in self.entity_list.iter_mut() {
            entity.borrow_mut().update();
        }
    }

    pub fn render(&self, renderer: &mut Renderer) {
        for entity in self.entity_list.iter().rev() {
            let EntityDrawInfo {
                world_rect,
                texture,
                texture_idx,
            } = entity.borrow().get_draw_info();

            let view_rect = self.scene_info.camera.transform(world_rect);

            match renderer.clip(view_rect) {
                Some(screen_rect) => {
                    texture.draw_idx(&mut renderer.canvas, screen_rect, texture_idx);
                }
                None => (),
            }
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
        self.entity_list.get(0).unwrap().borrow().pos()
    }
    pub fn set_position(&mut self, pos: Vec2) {
        self.entity_list.get(0).unwrap().borrow_mut().set_pos(pos);
    }

    pub fn toggle_camera_attachment(&mut self) {
        if self.scene_info.camera.attachment.is_some() {
            self.scene_info.camera.detach();
        } else {
            self.scene_info
                .camera
                .attach(self.entity_list.get(0).unwrap().clone());
        }
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
