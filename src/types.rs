use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Default)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Rect {
    pub fn point_left_bottom(&self) -> Point {
        Point {
            x: self.x,
            y: self.y,
        }
    }

    pub fn point_right_top(&self) -> Point {
        Point {
            x: self.x + self.w as i32,
            y: self.y + self.h as i32,
        }
    }

    /// Get the center point of the rect.
    pub fn point_center(&self) -> Point {
        Point {
            x: self.x + (self.w / 2) as i32,
            y: self.y + (self.h / 2) as i32,
        }
    }

    pub fn size(&self) -> Size {
        Size {
            w: self.w,
            h: self.h,
        }
    }

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

    pub fn transform(self, offset: Size) -> Self {
        Self {
            x: self.x + offset.w as i32,
            y: self.y + offset.h as i32,
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
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Into<Size> for Point {
    fn into(self) -> Size {
        Size {
            w: self.x,
            h: self.y,
        }
    }
}

impl std::ops::Add<Size> for Point {
    type Output = Point;

    fn add(self, rhs: Size) -> Self::Output {
        Point {
            x: self.x + rhs.w,
            y: self.y + rhs.h,
        }
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Default)]
pub struct Size {
    pub w: i32,
    pub h: i32,
}

impl Into<Point> for Size {
    fn into(self) -> Point {
        Point {
            x: self.w,
            y: self.h,
        }
    }
}

impl std::ops::Neg for Size {
    type Output = Size;

    fn neg(self) -> Self::Output {
        Size {
            w: -self.w,
            h: -self.h,
        }
    }
}

impl std::ops::Mul<i32> for Size {
    type Output = Size;

    fn mul(self, rhs: i32) -> Self::Output {
        Size {
            w: self.w * rhs,
            h: self.h * rhs,
        }
    }
}

impl std::ops::Div<i32> for Size {
    type Output = Size;

    // integer division for each component
    fn div(self, rhs: i32) -> Self::Output {
        Size {
            w: self.w / rhs,
            h: self.h / rhs,
        }
    }
}
