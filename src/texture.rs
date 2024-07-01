use std::fs;
use std::path::Path;

use crate::error::Error;

use sdl2::render::TextureCreator;
use sdl2::video::{Window, WindowContext};
use sdl2::{image::LoadTexture, render::Canvas};

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
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

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct Size {
    pub w: u32,
    pub h: u32,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct SubTexturePosition {
    frame: Rect,
    spriteSourceSize: Rect,
    sourceSize: Size,
}

#[derive(Serialize, Deserialize, Debug)]
struct FileMetadata {
    image: String,
    size: Size,
}

#[derive(Serialize, Deserialize, Debug)]
struct Metadata {
    frames: Vec<SubTexturePosition>,
    meta: FileMetadata,
}

/// sdl2 texture and its subtexture positions
pub struct Texture<'a> {
    sdl_texture: sdl2::render::Texture<'a>,
    positions: Vec<SubTexturePosition>,
}

impl<'a> Texture<'a> {
    /// load image from JSON metadata
    pub fn load_from_json(
        texture_creator: &'a TextureCreator<WindowContext>,
        path: &Path,
    ) -> Result<Self, Error> {
        let meta_str = fs::read_to_string(path).map_err(|_| Error::FileReadFailure)?;
        let metadata: Metadata =
            serde_json::from_str(meta_str.as_str()).map_err(|_| Error::JSONParseFailure)?;

        let sdl_texture = texture_creator
            .load_texture(path.parent().unwrap().join(metadata.meta.image))
            .map_err(|_| Error::TextureCreateFailure)?;

        Ok(Self {
            sdl_texture,
            positions: metadata.frames,
        })
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, position: Rect, idx: usize) {
        let texture_position: &SubTexturePosition = self.positions.get(idx).unwrap();
        let from_rect: Rect = texture_position.frame;

        let width_ratio: f32 = position.w as f32 / texture_position.sourceSize.w as f32;
        let height_ratio: f32 = position.h as f32 / texture_position.sourceSize.h as f32;
        let to_rect = Rect {
            x: position.x
                + (texture_position.spriteSourceSize.x as f32 * width_ratio).round() as i32,
            y: position.y
                + (texture_position.spriteSourceSize.y as f32 * height_ratio).round() as i32,
            w: (texture_position.spriteSourceSize.w as f32 * width_ratio).round() as u32,
            h: (texture_position.spriteSourceSize.h as f32 * height_ratio).round() as u32,
        };
        // canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 0, 0));
        // canvas.draw_rect(position.clone().into()).unwrap();
        // canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 255));
        // canvas.draw_rect(to_rect.clone().into()).unwrap();
        canvas
            .copy::<sdl2::rect::Rect, sdl2::rect::Rect>(
                &self.sdl_texture,
                from_rect.into(),
                to_rect.into(),
            )
            .unwrap();
    }
}
