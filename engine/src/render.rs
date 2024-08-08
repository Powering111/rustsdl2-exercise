use crate::types::*;
use font::Font;
use manager::TextureManager;

pub mod font;
pub mod manager;
pub mod texture;

/// Type alias for sdl2 canvas
pub type Canvas<'a> = &'a mut sdl2::render::Canvas<sdl2::video::Window>;

/// Information for every render process
pub struct RenderInfo {
    pub screen_size: Vec2,
    pub frame: usize,
}

pub struct Renderer {
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,

    pub texture_manager: manager::TextureManager,
    pub render_info: RenderInfo,

    fonts: Vec<Font>,
}

impl Renderer {
    pub(crate) fn new(video_subsystem: &sdl2::VideoSubsystem) -> Self {
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

        // init canvas
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.present();

        // render info
        let (window_width, window_height) = canvas.window().size();
        let render_info = RenderInfo {
            screen_size: Vec2 {
                x: window_width as i32,
                y: window_height as i32,
            },
            frame: 0,
        };

        let texture_creator = canvas.texture_creator();
        let mut texture_manager = TextureManager::new(texture_creator);

        // load font now
        // TODO: move to API

        texture_manager
            .load("font", std::path::Path::new("assets/font.json"))
            .unwrap();
        let font0 = crate::render::font::load_font(
            texture_manager.get("font"),
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 .,!?",
        )
        .unwrap();
        Self {
            canvas,
            texture_manager,
            render_info,
            fonts: vec![font0],
        }
    }

    pub(crate) fn render(&mut self, scene: &crate::game::scene::Scene) {
        // clear canvas
        self.canvas
            .set_draw_color(sdl2::pixels::Color::RGB(200, 150, 250));
        self.canvas.clear();

        scene.render(self);

        self.canvas.present();
        self.render_info.frame += 1;
    }

    pub(crate) fn set_screen_size(&mut self, size: Vec2) {
        self.render_info.screen_size = size;
    }
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
