use crate::game::scene::SceneInfo;
use crate::render::font::Font;
use crate::render::{Canvas, RenderInfo};
use crate::types::*;

use crate::game::ui::UIElement;

pub struct TextElement<'a> {
    font: Font<'a>,
    text: String,
    pos: Vec2,
}

impl<'a> UIElement for TextElement<'a> {
    fn draw(&self, canvas: Canvas, render_info: &RenderInfo, scene_info: &SceneInfo) {}
}
