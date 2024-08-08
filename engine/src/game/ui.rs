use crate::game::scene::SceneInfo;
use crate::Renderer;

pub mod text;

pub trait UIElement {
    fn draw(&self, renderer: &mut Renderer, scene_info: &SceneInfo);
}
