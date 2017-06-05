extern crate sdl2;

mod emscripten;
mod engine;
mod scenes;
mod board;
mod game;

use game::GameScene;
use engine::engine::{Engine, EngineMeta};

pub const SPEED: usize = 100;
pub const TITLE: &'static str = "Game of Life";


#[cfg(target_os = "emscripten")]
mod consts {
    pub const WIDTH: usize = 60;
    pub const HEIGHT: usize = 40;
    pub const SCALE: usize = 12;
}


#[cfg(not(target_os = "emscripten"))]
mod consts {
    pub const WIDTH: usize = 160;
    pub const HEIGHT: usize = 100;
    pub const SCALE: usize = 8;
}

use consts::*;

fn main() {
    let context = sdl2::init().expect("Trouple initializing sdl2.");
    let meta = EngineMeta::new(TITLE, WIDTH as u32, HEIGHT as u32, SCALE as u32);
    let mut game = Engine::new(context, meta);
    game.add_scene(Box::new(GameScene::new(WIDTH, HEIGHT, SCALE, SPEED)));


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
