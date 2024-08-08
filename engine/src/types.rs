use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Default)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Rect {
    pub fn point_left_bottom(&self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.y,
        }
    }

    pub fn point_right_top(&self) -> Vec2 {
        Vec2 {
            x: self.x + self.w as i32,
            y: self.y + self.h as i32,
        }
    }

    /// Get the center point of the rect.
    pub fn point_center(&self) -> Vec2 {
        Vec2 {
            x: self.x + (self.w / 2) as i32,
            y: self.y + (self.h / 2) as i32,
        }
    }

    pub fn size(&self) -> Vec2 {
        Vec2 {
            x: self.w,
            y: self.h,
        }
    }

    pub fn from_start_size(start: Vec2, size: Vec2) -> Self {
        Self {
            x: start.x,
            y: start.y,
            w: size.x,
            h: size.y,
        }
    }

    pub fn from_center_size(center: Vec2, size: Vec2) -> Self {
        Self {
            x: center.x - (size.x / 2) as i32,
            y: center.y - (size.y / 2) as i32,
            w: size.x,
            h: size.y,
        }
    }

    pub fn transform(self, offset: Vec2) -> Self {
        Self {
            x: self.x + offset.x,
            y: self.y + offset.y,
            w: self.w,
            h: self.h,
        }
    }

    /// Scale rect while center point is fixed.
    pub fn scale_up(self, scale: i32) -> Self {
        Rect::from_center_size(self.point_center(), self.size() * scale)
    }

    /// Scale rect while center point is fixed.
    /// It uses integer division.
    /// use `scale_up` and `scale_down` together to achieve rational number scaling.
    pub fn scale_down(self, scale: i32) -> Self {
        Rect::from_center_size(self.point_center(), self.size() / scale)
    }

    pub fn collides(&self, other: &Rect) -> bool {
        self.x < other.x + other.w
            && self.x + self.w > other.x
            && self.y < other.y + other.h
            && self.y + self.h > other.y
    }
}

impl Into<sdl2::rect::Rect> for Rect {
    fn into(self) -> sdl2::rect::Rect {
        sdl2::rect::Rect::new(self.x, self.y, self.w as u32, self.h as u32)
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Default)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Into<sdl2::rect::Point> for Vec2 {
    fn into(self) -> sdl2::rect::Point {
        sdl2::rect::Point::new(self.x, self.y)
    }
}

impl std::ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Self::Output {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl std::ops::Mul<i32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: i32) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec2 {
            x: (self.x as f32 * rhs) as i32,
            y: (self.y as f32 * rhs) as i32,
        }
    }
}

impl std::ops::Div<i32> for Vec2 {
    type Output = Vec2;

    // integer division for each component
    fn div(self, rhs: i32) -> Self::Output {
        Vec2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
