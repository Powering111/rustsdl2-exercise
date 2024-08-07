use crate::types::*;

pub mod manager;

pub mod font;
pub mod texture;

/// Type alias for sdl2 canvas
pub type Canvas<'a> = &'a mut sdl2::render::Canvas<sdl2::video::Window>;

/// Information for every render process
pub struct RenderInfo {
    pub screen_size: Size,
}

/// transforms rect from view space to screen space.
/// returns Some if the rect is visible
pub fn clip(view_rect: Rect, screen_size: Size) -> Option<Rect> {
    let mut transformed_rect = view_rect.transform(Size {
        w: screen_size.w / 2,
        h: screen_size.h / 2,
    });
    transformed_rect.y = screen_size.h - transformed_rect.y;
    if transformed_rect.collides(&Rect::from_start_size(Point::default(), screen_size)) {
        Some(transformed_rect)
    } else {
        None
    }
}
