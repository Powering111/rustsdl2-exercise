use std::path::Path;

use sdl2::image::LoadTexture;
use sdl2::{event::Event, pixels::Color};

use font::Font;
use texture::{Point, Size, Rect};

extern crate sdl2;

mod error;
mod font;
mod texture;

fn main() {
    let sdl_context = sdl2::init().expect("sdl2 initialization failed");
    let video_subsystem = sdl_context.video().expect("video subsystem failed");

    let window = video_subsystem
        .window("example title", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .unwrap();

    let texture_creator = canvas.texture_creator();

    let texture0 =
        texture::Texture::load_from_json(&texture_creator, Path::new("assets/font.json")).unwrap();

    let texture1 = texture_creator
        .load_texture(Path::new("assets/human.bmp"))
        .unwrap();

    let font0 = Font::load(&texture_creator, Path::new("assets/font.json"), "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 !?").unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut frames: u64 = 0;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut anim_index = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { .. } => {}
                Event::MouseButtonDown { .. } => {}
                _ => {}
            }
        }
        canvas.set_draw_color(Color::RGB(250, (((frames) % 100)+100) as u8, 250));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.draw_rect(sdl2::rect::Rect::new(20, 20, 500, 500)).unwrap();
        canvas
            .copy(
                &texture1,
                sdl2::rect::Rect::new(anim_index * 64, 0, 64, 64),
                sdl2::rect::Rect::new(20, 20, 500, 500),
            )
            .unwrap();

        texture0.draw(
            &mut canvas,
            texture::Rect {
                x: 50,
                y: 50,
                w: 100,
                h: 200,
            },
            0,
        );
        texture0.draw(
            &mut canvas,
            texture::Rect {
                x: 150,
                y: 50,
                w: 100,
                h: 200,
            },
            1,
        );
        texture0.draw(
            &mut canvas,
            texture::Rect {
                x: 250,
                y: 50,
                w: 100,
                h: 200,
            },
            2,
        );
        
        font0.draw(&mut canvas, "hello world!", Point {x: 30, y: 450}, Size {w: 50, h: 100});

        canvas.present();

        frames += 1;

        if frames % 15 == 0 {
            anim_index = (anim_index + 1) % 6;
        }
    }
}
