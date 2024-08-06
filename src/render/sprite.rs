use sdl2::{render::Canvas, video::Window};

use super::texture::{Point, Rect, Size, Texture};

pub struct Sprite<'a> {
    texture: Texture<'a>,
    duration_per_frame: f32,
}

impl<'a> Sprite<'a> {
    pub fn new(texture: Texture<'a>) -> Self {
        Self {
            texture: texture,
            duration_per_frame: 0.2f32,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, position: Point, scale: Size) {
        self.texture.draw_idx(
            canvas,
            Rect {
                x: position.x,
                y: position.y,
                w: scale.w,
                h: scale.h,
            },
            0,
        );
    }
}
