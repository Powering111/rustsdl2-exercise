use std::path::Path;

use sdl2::mixer::InitFlag;
use sdl2::{event::Event, pixels::Color};

use render::font::Font;
use render::texture::{self, Point, Rect, Size};

extern crate sdl2;

mod error;
mod render;

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
        texture::load_from_json(&texture_creator, Path::new("assets/font.json")).unwrap();

    let texture1 =
        texture::load_from_file(&texture_creator, Path::new("assets/human.bmp")).unwrap();

    let font0 = Font::load(
        &texture_creator,
        Path::new("assets/font.json"),
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 .,!?",
    )
    .unwrap();

    let audio_subsystem = sdl_context.audio().unwrap();

    sdl2::mixer::open_audio(
        44100,
        sdl2::mixer::AUDIO_S16LSB,
        sdl2::mixer::DEFAULT_CHANNELS,
        64,
    )
    .unwrap();

    let mixer_context =
        sdl2::mixer::init(InitFlag::MP3 | InitFlag::FLAC | InitFlag::MOD | InitFlag::OGG).unwrap();

    sdl2::mixer::allocate_channels(4);

    let audio1 = sdl2::mixer::Music::from_file("assets/sound/sample.mp3").unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut frames: u64 = 0;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut anim_index = 0;
    let start_time = std::time::Instant::now();
    let mut last_elapsed = start_time.elapsed();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { .. } => {
                    audio1.play(1).unwrap();
                }
                Event::MouseButtonDown { .. } => {}
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(
            250,
            (((start_time.elapsed().as_millis()) % 1000) / 10 + 100) as u8,
            250,
        ));
        canvas.clear();

        font0.draw(&mut canvas, "hello world!\nlorem ipsum dolor sit amet,\nconsectetur adipisicing elit,\nsed do eiusmod tempor\nut labore et dolore magna aliqua.", Point {x: 30, y: 50}, Size {w: 20, h: 40});

        let elapsed = start_time.elapsed();
        let fps = 1f32 / (elapsed - last_elapsed).as_secs_f32();
        last_elapsed = elapsed;
        font0.draw(
            &mut canvas,
            format!("{fps:.0} FPS").as_str(),
            Point { x: 0, y: 0 },
            Size { w: 32, h: 64 },
        );
        texture1.draw(
            &mut canvas,
            Rect {
                x: 300,
                y: 10,
                w: 500,
                h: 100,
            },
        );

        canvas.present();

        frames += 1;

        if frames % 15 == 0 {
            anim_index = (anim_index + 1) % 6;
        }
    }
}
