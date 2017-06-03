extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;

mod board;
mod game;

use game::Game;


const WIDTH: u32 = 120;
const HEIGHT: u32 = 80;
const SCALE: u32 = 10;
const SPEED: usize = 120;

const DEAD_COLOR: (u8, u8, u8) = (0, 28, 67);
const ALIVE_COLOR: (u8, u8, u8) = (255, 153, 0);
const BG_COLOR: (u8, u8, u8) = (0, 0, 150);


fn main() {
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();
    let window = video_ctx.window("Game of Life", WIDTH * SCALE, HEIGHT * SCALE)
                          .position_centered().opengl().build().unwrap();
    let mut renderer = window.renderer().accelerated().build().unwrap();
    let event_pump = ctx.event_pump().unwrap();

    let mut game = Game::new(event_pump, SPEED, WIDTH as usize, HEIGHT as usize, SCALE as usize);


    game.set_draw_callback(Box::new(move |cells| {
        renderer.set_draw_color(Color::RGB(BG_COLOR.0, BG_COLOR.1, BG_COLOR.2));
        renderer.clear();
        for (y, row) in cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if cell.is_alive() {
                    renderer.set_draw_color(Color::RGB(ALIVE_COLOR.0, ALIVE_COLOR.1, ALIVE_COLOR.2));
                    renderer.fill_rect(
                        Rect::new(x as i32 * SCALE as i32,
                                  y as i32 * SCALE as i32,
                                  SCALE - 1 as u32, SCALE - 1 as u32)).unwrap();
                } else {
                    renderer.set_draw_color(Color::RGB(DEAD_COLOR.0, DEAD_COLOR.1, DEAD_COLOR.2));
                    renderer.fill_rect(
                        Rect::new(x as i32 * SCALE as i32,
                                  y as i32 * SCALE as i32,
                                  SCALE - 1 as u32, SCALE - 1 as u32)).unwrap();
                }
            }
        }
        renderer.present();
    }));
    game.run();
}
