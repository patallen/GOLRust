extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::EventPump;

use std::io::Read;
use std::fs::File;


mod board;
use board::Game;


const WIDTH: u32 = 120;
const HEIGHT: u32 = 90;
const SCALE: u32 = 8;
const SPEED: usize = 150;

const DEAD_COLOR: (u8, u8, u8) = (0, 28, 67);
const ALIVE_COLOR: (u8, u8, u8) = (255, 153, 0);
const BG_COLOR: (u8, u8, u8) = (0, 0, 150);


fn main() {
    let mut file = File::open("/Users/patallen/Code/Rust/game_of_life/src/states/first.txt").unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer);
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();
    let window = video_ctx.window("Game of Life", WIDTH * SCALE, HEIGHT * SCALE)
                          .position_centered().opengl().build().unwrap();
    let mut renderer = window.renderer().accelerated().build().unwrap();
    let event_pump = ctx.event_pump().unwrap();
    let mut game = Game::new(buffer, event_pump, SPEED, WIDTH as usize, HEIGHT as usize, SCALE as usize);


    game.set_draw_callback(Box::new(move |cells| {
        renderer.set_draw_color(Color::RGB(BG_COLOR.0, BG_COLOR.1, BG_COLOR.2));
        renderer.clear();
        for cell in cells {
            if cell.alive {
                renderer.set_draw_color(Color::RGB(ALIVE_COLOR.0, ALIVE_COLOR.1, ALIVE_COLOR.2));
                renderer.fill_rect(
                    Rect::new(cell.x as i32 * SCALE as i32,
                              cell.y as i32 * SCALE as i32,
                              SCALE - 1 as u32, SCALE - 1 as u32)).unwrap();
            } else {
                renderer.set_draw_color(Color::RGB(DEAD_COLOR.0, DEAD_COLOR.1, DEAD_COLOR.2));
                renderer.fill_rect(
                    Rect::new(cell.x as i32 * SCALE as i32,
                                cell.y as i32 * SCALE as i32,
                                SCALE - 1 as u32, SCALE - 1 as u32)).unwrap();
            }
        }
        renderer.present();

    }));
    game.run();
}
