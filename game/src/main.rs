use std::path::Path;


use engine::Engine;
use engine::game::{self, scene::Scene};
use engine::types::*;

extern crate engine;

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
            scene0.add_entity(game::entity::HumanEntity::new(
                engine.get_texture("sprite.human"),
                Vec2 {
                    x: x * 200 - 2000,
                    y: y * 200 - 2000,
                },
            ));
        }
    }
    scene0.add_entity(game::entity::HumanEntity::new(
        engine.get_texture("sprite.test"),
        Vec2 { x: 0, y: 0 },
    ));

    engine.add_scene(scene0);

    // main loop
    engine.main_loop();

    println!("finish");
}
