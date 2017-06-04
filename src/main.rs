extern crate sdl2;

mod emscripten;
mod engine;
mod board;
mod game;

use game::GameScene;

pub const SPEED: usize = 100;

#[cfg(target_os = "emscripten")]
mod consts {
    pub const WIDTH: u32 = 60;
    pub const HEIGHT: u32 = 40;
    pub const SCALE: u32 = 12;
}


#[cfg(not(target_os = "emscripten"))]
mod consts {
    pub const WIDTH: u32 = 160;
    pub const HEIGHT: u32 = 100;
    pub const SCALE: u32 = 8;
}

use consts::*;

fn main() {
    let context = sdl2::init().expect("Trouple initializing sdl2");
    let meta = engine::engine::EngineMeta::new("GOL", WIDTH, HEIGHT, SCALE);
    let mut game = engine::engine::Engine::new(context, meta);
    game.add_scene(Box::new(GameScene::new(WIDTH as usize, HEIGHT as usize, SCALE as usize, SPEED)));


    let mut main_loop = || {
        game.handle_events();
        game.update();
        game.render();
    };

    #[cfg(target_os = "emscripten")]
    emscripten::set_main_loop_callback(main_loop);

    #[cfg(not(target_os = "emscripten"))]
    loop { (main_loop)(); }
}
