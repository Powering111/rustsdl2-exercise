use std::path::Path;

use game::scene::Scene;
use render::manager::TextureManager;
use sdl2::keyboard::Keycode;
use sdl2::mixer::InitFlag;
use sdl2::{event::Event, pixels::Color};

use render::font::load_font;
use render::{texture, RenderInfo};
use types::*;

extern crate sdl2;

mod error;
mod game;
mod render;
mod types;

fn main() {
    let sdl_context = sdl2::init().expect("sdl2 initialization failed");
    let video_subsystem = sdl_context.video().expect("video subsystem failed");

    let window = video_subsystem
        .window("example title", 800, 600)
        .position_centered()
        .allow_highdpi()
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .unwrap();

    let texture_creator = canvas.texture_creator();
    let mut texture_manager = TextureManager::new();

    texture_manager
        .load(&texture_creator, "font", Path::new("assets/font.json"))
        .expect("loading font failed");
    texture_manager
        .load(
            &texture_creator,
            "sprite.human",
            Path::new("assets/human.json"),
        )
        .expect("loading sprite.human failed");
    texture_manager
        .load(
            &texture_creator,
            "sprite.test",
            Path::new("assets/test.json"),
        )
        .expect("loading sprite.test failed");

    let font0 = load_font(
        texture_manager.get("font"),
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

    let (window_width, window_height) = canvas.window().size();
    let mut render_info = RenderInfo {
        screen_size: Vec2 {
            x: window_width as i32,
            y: window_height as i32,
        },
        frame: 0,
    };

    let mut scene0 = Scene::new();
    for x in 1..10 {
        for y in 1..10 {
            scene0.add_entity(game::entity::HumanEntity::new(
                &texture_manager,
                "sprite.human",
                Vec2 {
                    x: x * 200 - 1000,
                    y: y * 200 - 1000,
                },
            ));
        }
    }
    scene0.add_entity(game::entity::HumanEntity::new(
        &texture_manager,
        "sprite.test",
        Vec2 { x: 0, y: 0 },
    ));

    let mut anim_index = 0;
    let start_time = std::time::Instant::now();
    let mut last_elapsed = start_time.elapsed();

    // init canvas
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    canvas.present();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::Window { win_event, .. } => match win_event {
                    sdl2::event::WindowEvent::Resized(x, y) => {
                        render_info.screen_size = Vec2 { x, y }
                    }
                    _ => (),
                },
                Event::KeyDown { keycode, .. } => match keycode {
                    Some(keycode) => match keycode {
                        Keycode::SPACE => audio1.play(1).unwrap(),
                        _ => (),
                    },
                    None => (),
                },
                Event::MouseWheel { precise_y, .. } => {
                    scene0.add_zoom(precise_y);
                }
                Event::MouseButtonDown { .. } => {}
                _ => {}
            }
        }

        for keycode in event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
        {
            match keycode {
                Keycode::W => scene0.set_position(scene0.get_position() + Vec2 { x: 0, y: 10 }),
                Keycode::A => scene0.set_position(scene0.get_position() + Vec2 { x: -10, y: 0 }),
                Keycode::S => scene0.set_position(scene0.get_position() + Vec2 { x: 0, y: -10 }),
                Keycode::D => scene0.set_position(scene0.get_position() + Vec2 { x: 10, y: 0 }),
                _ => (),
            }
        }

        // clear canvas
        canvas.set_draw_color(Color::RGB(
            250,
            (((start_time.elapsed().as_millis()) % 1000) / 10 + 100) as u8,
            250,
        ));
        canvas.clear();

        // update game
        scene0.update();

        // render the scene
        scene0.render(&mut canvas, &render_info);

        // debug
        font0.draw(&mut canvas, "hello world!\nlorem ipsum dolor sit amet,\nconsectetur adipisicing elit,\nsed do eiusmod tempor\nut labore et dolore magna aliqua.", Vec2 {x: 30, y: 50}, Vec2 {x: 20, y: 40});

        let elapsed = start_time.elapsed();
        let fps = 1f32 / (elapsed - last_elapsed).as_secs_f32();
        last_elapsed = elapsed;
        font0.draw(
            &mut canvas,
            format!("{fps:.0} FPS").as_str(),
            Vec2 { x: 0, y: 0 },
            Vec2 { x: 32, y: 64 },
        );

        // show rendered screen
        canvas.present();

        render_info.frame += 1;
    }
}
