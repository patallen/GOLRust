extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;

use std::io::Read;
use std::fs::File;


mod board;
use board::Game;


const WIDTH: u32 = 80;
const HEIGHT: u32 = 80;
const SCALE: u32 = 10;


fn main() {
    let mut file = File::open("/Users/patallen/Code/Rust/game_of_life/src/states/first.txt").unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer);
    let mut game = Game::new(buffer, 10);

    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();
    let window = video_ctx.window("GOL", WIDTH * SCALE, HEIGHT * SCALE)
                          .position_centered().opengl().build().unwrap();
    let mut renderer = window.renderer().accelerated().build().unwrap();

    game.set_draw_callback(Box::new(move |cells| {
        renderer.set_draw_color(Color::RGB(255, 255, 255));
        renderer.clear();
        renderer.set_draw_color(Color::RGB(0, 0, 0));
        for cell in cells {
            if cell.alive {
                renderer.fill_rect(
                    Rect::new(cell.x as i32 * SCALE as i32,
                              cell.y as i32 * SCALE as i32,
                              SCALE as u32, SCALE as u32)).unwrap();
            }
        }
        renderer.present();

    }));
    game.run();
}
