pub use crate::Renderer;
pub use crate::game::scene::SceneInfo;

pub mod text;

pub trait UIElement {
    fn draw(&self, renderer: &mut Renderer, scene_info: &SceneInfo);
}
