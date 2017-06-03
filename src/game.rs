use std::{thread, time};
use std::process;

use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;
use board::{Board, Cell};

enum GameMode {
    Playing,
    Paused,
    Restart,
}

pub struct Game {
    cell_size: usize,
    pub board: Board,
    round: usize,
    speed: usize,
    draw_callback: Box<FnMut(Vec<Cell>)>,
    mode: GameMode,
    events: EventPump,
    do_draw: bool,
}


impl Game {
    pub fn new(events: EventPump, speed: usize, width: usize, height: usize, scale: usize) -> Game {
        Game {
            cell_size: scale,
            round: 0,
            board: Board::new(width, height),
            speed: speed,
            draw_callback: Box::new(|_| {}),
            mode: GameMode::Restart,
            events: events,
            do_draw: true,
        }
    }
    pub fn run(&mut self) {
        loop {
            match self.mode {
                GameMode::Playing => {
                    let q_sec = time::Duration::from_millis(300);
                    thread::sleep(q_sec);
                    self.step();
                },
                GameMode::Restart => { self.restart(); },
                _ => {}
            }
            self.handle_events();
            match self.do_draw {
                true => {self.draw_board(); self.do_draw = false;}
                false => {}
            }
        }
    }
    pub fn restart(&mut self) {
        self.board.clear();
        self.round = 0;
        self.do_draw = true;
        self.mode = GameMode::Paused;
    }
    pub fn step(&mut self) {
        self.board.update();
        self.draw_board();
    }

    fn draw_board(&mut self) {
        let cells = self.board.clone_cells();
        (self.draw_callback)(cells);
    }
    pub fn set_draw_callback(&mut self, func: Box<FnMut(Vec<Cell>)>) {
        self.draw_callback = func;
    }
    fn handle_events(&mut self) {
        for event in self.events.poll_iter() {
            match event {
                Event::Quit{..} => process::exit(1),
                Event::KeyDown{keycode: kc, ..} => match kc {
                    Some(Keycode::Escape) | Some(Keycode::Q) => process::exit(1),
                    Some(Keycode::R) => self.mode = GameMode::Restart,
                    Some(Keycode::P) => {
                        match self.mode {
                            GameMode::Paused => self.mode = GameMode::Playing,
                            GameMode::Playing => self.mode = GameMode::Paused,
                            _ => {}
                        }
                    },
                    _ => {}
                },
                Event::MouseButtonDown{mouse_btn: button, x, y, ..} => match &self.mode {
                    &GameMode::Paused => {
                        match button {
                            MouseButton::Left => {
                                let cell_idx = cell_from_xy(self.cell_size,
                                                            self.board.width(),
                                                            x as usize, y as usize);
                                self.board.set_cell_state(cell_idx);
                                self.do_draw = true;
                            },
                            MouseButton::Right => {
                                let cell_idx = cell_from_xy(self.cell_size,
                                                            self.board.width(),
                                                            x as usize, y as usize);
                                self.board.unset_cell_state(cell_idx);
                                self.do_draw = true;
                            },
                            _ => {}
                        }
                    },
                    _ => {}
                },
                _ => {}
            }
        }
    }
}

fn cell_from_xy(cell_size: usize, width: usize, x: usize, y: usize) -> usize {
    let adj_y = y / cell_size;
    let adj_x = x / cell_size;
    return adj_y * width + adj_x;
}
