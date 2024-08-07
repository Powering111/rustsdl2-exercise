use crate::render::Canvas;

use crate::game::scene::SceneInfo;
use crate::render::RenderInfo;

pub mod text;

pub trait UIElement {
    fn draw(&self, canvas: Canvas, render_info: &RenderInfo, scene_info: &SceneInfo);
}
