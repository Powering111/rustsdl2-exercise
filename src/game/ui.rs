use sdl2::render::Canvas;
use sdl2::video::Window;

pub trait UIElement {
    fn draw(&self, canvas: &mut Canvas<Window>);
}
