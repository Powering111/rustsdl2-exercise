use crate::game::scene::SceneInfo;
use crate::game::ui::UIElement;
use crate::render::font::Font;
use crate::Renderer;

use crate::types::*;

pub struct TextElement {
    font: Font,
    pub text: String,
    pub pos: Vec2,
    pub scale: Vec2,
}

impl TextElement {
    pub fn new(font: Font) -> Self {
        Self {
            font,
            text: String::from("Hello world!"),
            pos: Vec2 { x: 0, y: 0 },
            scale: Vec2 { x: 100, y: 100 },
        }
    }
}

impl UIElement for TextElement {
    fn draw(&self, renderer: &mut Renderer, scene_info: &SceneInfo) {
        self.font.draw(
            &mut renderer.canvas,
            self.text.as_str(),
            self.pos,
            self.scale,
        );
    }
}
