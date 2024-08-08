use std::path::Path;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub use error::Error;
use game::scene::Scene;
use render::texture::Texture;
use render::Renderer;
use types::*;

mod error;
pub mod game;
mod render;
pub mod types;

/// Game engine.
pub struct Engine {
    renderer: render::Renderer,

    // TODO: change current_scene to reference
    current_scene: usize,
    scenes: Vec<Scene>,

    event_pump: sdl2::EventPump,

    start_time: std::time::Instant,
    last_elapsed: std::time::Duration,
}

impl Engine {
    /// Creates new window and initialize everything.
    pub fn new() -> Result<Self, Error> {
        let sdl_context = sdl2::init()
            .map_err(|err| Error::InitFailure(format!("sdl2 initialization failed : {err}")))?;

        let video_subsystem = sdl_context.video().map_err(|err| {
            Error::InitFailure(format!("video subsystem initialization failed : {err}"))
        })?;
        let audio_subsystem = sdl_context.audio().map_err(|err| {
            Error::InitFailure(format!("audio subsystem initialization failed : {err}"))
        })?;

        let renderer = render::Renderer::new(&video_subsystem);
        let event_pump = sdl_context.event_pump().unwrap();

        Ok(Self {
            renderer,
            current_scene: 0,
            scenes: Vec::new(),
            event_pump,
            start_time: std::time::Instant::now(),
            last_elapsed: std::time::Duration::new(0, 0),
        })
    }

    /// update the current scene
    fn update(&mut self) {
        if let Some(scene) = self.scenes.get_mut(self.current_scene) {
            scene.update();
        } else {
            panic!("no scene");
        }
    }

    // render the current scene
    fn render(&mut self) {
        if let Some(scene) = self.scenes.get(self.current_scene) {
            self.renderer.render(scene);
        } else {
            panic!("no scene");
        }
    }

    /// loads texture named *name* from *path*.
    /// when file extension is `.json`, the JSON metadata is loaded together.
    pub fn load_texture(&mut self, name: &'static str, path: &Path) {
        self.renderer.texture_manager.load(name, path).unwrap();
    }

    /// Get Texture by name.
    /// Texture should have been loaded.
    pub fn get_texture(&mut self, name: &'static str) -> Texture {
        self.renderer.texture_manager.get(name)
    }

    pub fn main_loop(mut self) {
        loop {
            let curr_scene = self.scenes.get_mut(self.current_scene).unwrap();
            // TODO: event handler pattern
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => return,
                    Event::Window { win_event, .. } => match win_event {
                        sdl2::event::WindowEvent::Resized(x, y) => {
                            println!("Screen resized: ({x},{y})");
                            self.renderer.set_screen_size(Vec2 { x, y });
                        }
                        _ => (),
                    },
                    Event::MouseWheel { precise_y, .. } => {
                        curr_scene.add_zoom(precise_y);
                    }
                    Event::MouseButtonDown { .. } => {}
                    _ => {}
                }
            }

            for keycode in self
                .event_pump
                .keyboard_state()
                .pressed_scancodes()
                .filter_map(Keycode::from_scancode)
            {
                match keycode {
                    Keycode::W => {
                        curr_scene.set_position(curr_scene.get_position() + Vec2 { x: 0, y: 10 })
                    }
                    Keycode::A => {
                        curr_scene.set_position(curr_scene.get_position() + Vec2 { x: -10, y: 0 })
                    }
                    Keycode::S => {
                        curr_scene.set_position(curr_scene.get_position() + Vec2 { x: 0, y: -10 })
                    }
                    Keycode::D => {
                        curr_scene.set_position(curr_scene.get_position() + Vec2 { x: 10, y: 0 })
                    }
                    _ => (),
                }
            }

            // update game
            self.update();

            // render the scene
            self.render();

            let elapsed = self.start_time.elapsed();
            let fps = 1f32 / (elapsed - self.last_elapsed).as_secs_f32();
            self.last_elapsed = elapsed;
            // self.renderer.font.draw(
            //     &mut canvas,
            //     format!("{fps:.0} FPS").as_str(),
            //     Vec2 { x: 0, y: 0 },
            //     Vec2 { x: 32, y: 64 },
            // );
        }
    }

    pub fn add_scene(&mut self, scene: Scene) {
        self.scenes.push(scene)
    }
}
