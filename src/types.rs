use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Default)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

impl Rect {
    pub fn from_start_size(start: Point, size: Size) -> Self {
        Self {
            x: start.x,
            y: start.y,
            w: size.w,
            h: size.h,
        }
    }

    pub fn from_center_size(center: Point, size: Size) -> Self {
        Self {
            x: center.x - (size.w / 2) as i32,
            y: center.y - (size.h / 2) as i32,
            w: size.w,
            h: size.h,
        }
    }
}

impl std::ops::Add<Size> for Rect {
    type Output = Rect;
    fn add(self, rhs: Size) -> Self::Output {
        Self::Output {
            x: self.x + rhs.w as i32,
            y: self.y + rhs.h as i32,
            w: self.w,
            h: self.h,
        }
    }
}

impl Into<sdl2::rect::Rect> for Rect {
    fn into(self) -> sdl2::rect::Rect {
        sdl2::rect::Rect::new(self.x, self.y, self.w, self.h)
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Default)]
pub struct Size {
    pub w: u32,
    pub h: u32,
}
