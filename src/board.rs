use std::process;
use std::fmt;
use std::{thread, time};

use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

const OFFSETS: [(isize, isize); 8] = [
    (0, -1), (1, -1), (1, 0), (1, 1),
    (0, 1), (-1, 1), (-1, 0), (-1, -1),
];

#[derive(Clone, Debug)]
struct Cell {
    pub alive: bool,
    pub x: isize,
    pub y: isize,
}
impl Cell {
    fn new(x: isize, y: isize, alive: bool) -> Cell {
        Cell {
            x: x,
            y: y,
            alive: alive
        }
    }
    fn kill(&mut self) {
        self.alive = false;
    }
    fn revive(&mut self) {
        self.alive = true;
    }
}

pub struct Board {
    cells: Vec<Cell>,
    width: isize,
    height: isize,
}

impl Board {
    fn update(&mut self) {
        let next = self.cells.clone();
        for (idx, cell) in next.iter().enumerate() {
            let mut alive_neighbors = 0;
            for offset in OFFSETS.iter() {
                let nx = cell.x + offset.0;
                let ny = cell.y + offset.1;
                if ny >= 0 && ny < self.height && nx >= 0 && nx < self.width {
                    let index = self.width * ny + nx;
                    if next[index as usize].alive {
                        alive_neighbors += 1;
                    }
                }
            }
            match (self.cells[idx as usize].alive, alive_neighbors) {
                (true, 2) | (true, 3) => {}
                (false, 3) => { self.cells[idx as usize].revive() },
                _ => { self.cells[idx as usize].kill() }
            };
        }
    }
    pub fn set_cell_state(&mut self, cell: usize) {
        self.cells[cell].revive();
    }
    pub fn clear(&mut self) {
        self.cells.iter_mut().map(|c| c.kill());
    }
    pub fn new(source: String, width: usize, height: usize, scale: usize) -> Self {
        let mut cells: Vec<Cell> = Vec::new();
        for y in 0..height {
            for x in 0..width {
                cells.push(Cell::new(x as isize, y as isize, false))
            }
        }
        Board {
            height: height as isize,
            width: width as isize,
            cells: cells,
        }
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v: Vec<char> = self.cells.iter().map(|x| match x.alive { true => 'X', false => '-' }).collect();
        let fin = v.chunks(self.width as usize)
                   .map(|c| c.into_iter().collect::<String>())
                   .collect::<Vec<String>>().join("\n");
        write!(f, "{}", fin)
    }
}

enum GameMode {
    Playing,
    Paused,
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
    pub fn new(source: String, events: EventPump, speed: usize, width: usize, height: usize, scale: usize) -> Game {
        Game {
            cell_size: scale,
            round: 0,
            board: Board::new(source, width, height, scale),
            speed: speed,
            draw_callback: Box::new(|x| {}),
            mode: GameMode::Playing,
            events: events,
            do_draw: false,
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
        self.mode = GameMode::Paused;
    }
    pub fn step(&mut self) {
        self.board.update();
        self.draw_board();
    }
    fn draw_board(&mut self) {
        let cells = self.board.cells.clone();
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
                    Some(Keycode::P) => {
                        match self.mode {
                            GameMode::Paused => self.mode = GameMode::Playing,
                            GameMode::Playing => self.mode = GameMode::Paused,
                        }
                    },
                    _ => {}
                },
                Event::MouseButtonDown{mouse_btn: button, x: x, y: y, ..} => match &self.mode {
                    &GameMode::Paused => {
                        match button {
                            MouseButton::Left => {
                                let cell_idx = cell_from_xy(self.cell_size,
                                                            self.board.width as usize,
                                                            x as usize, y as usize);
                                self.board.set_cell_state(cell_idx);
                                self.do_draw = true;
                            },
                            MouseButton::Right => {
                                let cell_idx = cell_from_xy(self.cell_size,
                                                            self.board.width as usize,
                                                            x as usize, y as usize);
                            }
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
