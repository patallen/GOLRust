use std::fmt;
use std::{thread, time};

// A Cell is Alive or Dead
// 1. Any live cell with < 2 neighbors dies of loneliness
// 2. Any live cell with > 3 neighbors dies of overcrowding
// 3. Any Dead cell with exactly three live neighbors becomes a live cell
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
    pub fn new(source: String) -> Self {
        let mut cells: Vec<Cell> = Vec::new();
        let mut line_length = 0;
        for (lidx, line) in source.trim().split('\n').enumerate() {
            let mut ll = 0;
            for (cidx, ch) in line.chars().enumerate() {
                cells.push(match &*ch.to_lowercase().to_string() {
                    "x" | "-" => { ll += 1; Cell::new(cidx as isize, lidx as isize, false)},
                    "o" => { ll += 1; Cell::new(cidx as isize, lidx as isize, true)},
                    " " => {continue}
                    _ => panic!("{} is an invalid character.", ch)
                });
            }
            if line_length == 0 { line_length = ll; } else {
                if line_length != ll { panic!("Mixing line lengths") }
            }
        }
        Board {
            height: cells.len() as isize / line_length as isize,
            width: line_length as isize,
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
pub struct Game {
    pub board: Board,
    round: usize,
    speed: usize,
    draw_callback: Box<FnMut(Vec<Cell>)>
}

impl Game {
    pub fn new(source: String, speed: usize) -> Game {
        Game {
            round: 0,
            board: Board::new(source),
            speed: speed,
            draw_callback: Box::new(|x| {})
        }
    }
    pub fn run(&mut self) {
        loop {
            let q_sec = time::Duration::from_millis(300);
            thread::sleep(q_sec);
            self.step();
        }
    }
    pub fn step(&mut self) {
        self.board.update();
        let cells = self.board.cells.clone();
        (self.draw_callback)(cells);
    }
    pub fn set_draw_callback(&mut self, func: Box<FnMut(Vec<Cell>)>) {
        self.draw_callback = func;
    }
}
