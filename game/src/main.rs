use std::path::Path;

use engine::game::{self, scene::Scene};
use engine::types::*;
use engine::Engine;

fn main() {
    println!("start");
    let mut engine = Engine::new().unwrap();

    // load textures
    engine.load_texture("sprite.human", Path::new("assets/human.json"));
    engine.load_texture("sprite.test", Path::new("assets/test.json"));
    engine.load_texture("sprite.awesomeface", Path::new("assets/awesomeface_3d.png"));

    // new scene
    let mut scene0 = Scene::new();
    for x in 1..20 {
        for y in 1..20 {
            scene0.add_entity(Box::new(game::entity::HumanEntity::new(
                engine.get_texture("sprite.human"),
                Vec2 {
                    x: x * 200 - 2000,
                    y: y * 200 - 2000,
                },
            )));
        }
    }
    scene0.add_entity(Box::new(game::entity::HumanEntity::new(
        engine.get_texture("sprite.test"),
        Vec2 { x: 0, y: 0 },
    )));

    let mut debug_text =
        game::ui::text::TextElement::new(engine.renderer.fonts.get(0).unwrap().clone());
    debug_text.text = String::from("asdf");
    debug_text.pos = Vec2 { x: 100, y: 100 };
    debug_text.scale = Vec2 { x: 100, y: 200 };

    scene0.add_ui(Box::new(debug_text));

    engine.add_scene(scene0);

    // main loop
    engine.main_loop();

    println!("finish");
}
