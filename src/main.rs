use std::io::{self, Read};
use std::fs::File;

mod board;

use board::{ Board, Game };


fn main() {
    let mut file = File::open("/Users/patallen/Code/Rust/game_of_life/src/states/first.txt").unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer);
    let mut game = Game::new(buffer, 10);
    game.step();
    game.step();
}
