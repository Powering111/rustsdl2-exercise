use std::fs;
use std::path::Path;
use std::rc::Rc;

use crate::error::Error;
use crate::render::Canvas;
use crate::types::*;

use sdl2::image::LoadTexture;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Size {
    w: i32,
    h: i32,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct SubTexturePosition {
    /// subtexture region in the source image
    frame: Rect,
    /// region to draw in the destination rect
    spriteSourceSize: Rect,
    /// size of the destination rect
    sourceSize: Size,
}

#[derive(Serialize, Deserialize, Debug)]
struct FileMetadata {
    image: String,
    size: Size,
}

/// Requirement for the sprite sheet JSON file
/// It needs two key:
/// - *frames* : array of subtexture's position
///     each contain the key *frame*, which represents the rectangle.
///     - *frame* : composed of *x*, *y*, *w*, *h*
/// - *meta* : metadata for the target image file
/// For more detail, see example at `assets/font.json`.
#[derive(Serialize, Deserialize, Debug)]
struct Metadata {
    frames: Vec<SubTexturePosition>,
    meta: FileMetadata,
}

/// Basic texture abstraction.
/// Use this type to load, store and draw texture.
/// It must live within the range of the TextureCreator that created this texture.
pub type Texture = Rc<TextureInner>;

/// sdl2 texture and its subtexture positions
pub struct TextureInner {
    sdl_texture: Rc<sdl2::render::Texture>,
    positions: Vec<SubTexturePosition>,
}

/// load image texture from JSON metadata
/// the JSON metadata may be generated from Aseprite.
pub fn load_from_json(
    texture_creator: &TextureCreator<WindowContext>,
    path: &Path,
) -> Result<Texture, Error> {
    let meta_str = fs::read_to_string(path).map_err(|_| Error::FileReadFailure)?;
    let metadata: Metadata =
        serde_json::from_str(meta_str.as_str()).map_err(|_| Error::JSONParseFailure)?;

    let sdl_texture = texture_creator
        .load_texture(path.parent().unwrap().join(metadata.meta.image))
        .map_err(|_| Error::TextureCreateFailure)?;

    Ok(Rc::new(TextureInner {
        sdl_texture: Rc::new(sdl_texture),
        positions: metadata.frames,
    }))
}

/// load image texture that does not have JSON metadata.
pub fn load_from_file(
    texture_creator: &TextureCreator<WindowContext>,
    path: &Path,
) -> Result<Texture, Error> {
    let sdl_texture = texture_creator
        .load_texture(path)
        .map_err(|_| Error::TextureCreateFailure)?;
    let sdl2::render::TextureQuery { width, height, .. } = sdl_texture.query();
    Ok(Rc::new(TextureInner {
        sdl_texture: Rc::new(sdl_texture),
        positions: vec![SubTexturePosition {
            frame: Rect {
                x: 0,
                y: 0,
                w: width as i32,
                h: height as i32,
            },
            spriteSourceSize: Rect {
                x: 0,
                y: 0,
                w: width as i32,
                h: height as i32,
            },
            sourceSize: Size {
                w: width as i32,
                h: height as i32,
            },
        }],
    }))
}

impl TextureInner {
    /// Draw texture to the canvas.
    /// - *canvas* : the canvas to draw.
    /// - *rect* : position and size to be drawn in screen, pixel.
    pub fn draw(&self, canvas: Canvas, rect: Rect) {
        canvas
            .copy::<Option<_>, sdl2::rect::Rect>(&self.sdl_texture, None, rect.into())
            .unwrap();
    }

    /// Draw texture to the canvas.
    /// It is automatically trimmed and stretched.
    /// - *canvas* : the canvas to draw.
    /// - *rect* : position and size to be drawn in screen, pixel.
    /// - *idx* : the frame index to draw. starts from 0.
    pub fn draw_idx(&self, canvas: Canvas, rect: Rect, idx: usize) {
        if self.positions.is_empty() {
            self.draw(canvas, rect);
        } else {
            let texture_position: &SubTexturePosition = self
                .positions
                .get(idx)
                .expect(format!("texture index {idx:} not found").as_str());
            let from_rect: Rect = texture_position.frame;

            let width_ratio: f32 = rect.w as f32 / texture_position.sourceSize.w as f32;
            let height_ratio: f32 = rect.h as f32 / texture_position.sourceSize.h as f32;
            let to_rect = Rect {
                x: rect.x
                    + (texture_position.spriteSourceSize.x as f32 * width_ratio).round() as i32,
                y: rect.y
                    + (texture_position.spriteSourceSize.y as f32 * height_ratio).round() as i32,
                w: (texture_position.spriteSourceSize.w as f32 * width_ratio).round() as i32,
                h: (texture_position.spriteSourceSize.h as f32 * height_ratio).round() as i32,
            };

            // Debug: render region rectangle
            canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 255));
            canvas.draw_rect(to_rect.clone().into()).unwrap();

            // draw
            canvas
                .copy::<sdl2::rect::Rect, sdl2::rect::Rect>(
                    &self.sdl_texture,
                    from_rect.into(),
                    to_rect.into(),
                )
                .unwrap();
        }
    }

    pub fn len(&self) -> usize {
        self.positions.len()
    }
}
