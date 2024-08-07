use crate::types::*;

pub mod manager;

pub mod font;
pub mod texture;

/// Type alias for sdl2 canvas
pub type Canvas<'a> = &'a mut sdl2::render::Canvas<sdl2::video::Window>;

/// Information for every render process
pub struct RenderInfo {
    pub screen_size: Vec2,
    pub frame: usize,
}

/// transforms rect from view space to screen space.
/// returns `Some` if the rect is visible, 
/// `None` if the rect is outside of the screen.
pub fn clip(view_rect: Rect, screen_size: Vec2) -> Option<Rect> {
    let mut transformed_rect = view_rect.transform(Vec2 {
        x: screen_size.x / 2,
        y: screen_size.y / 2,
    });

    // flip vertically to change y-axis direction
    transformed_rect.y = screen_size.y - transformed_rect.y - transformed_rect.h;

    if transformed_rect.collides(&Rect::from_start_size(Vec2::default(), screen_size)) {
        Some(transformed_rect)
    } else {
        None
    }
}
