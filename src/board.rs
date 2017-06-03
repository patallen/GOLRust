use std::fmt;


const OFFSETS: [(isize, isize); 8] = [
    (0, -1), (1, -1), (1, 0), (1, 1),
    (0, 1), (-1, 1), (-1, 0), (-1, -1),
];

#[derive(Clone, Debug)]
pub struct Cell {
    pub alive: bool,
    pub x: usize,
    pub y: usize,
}
impl Cell {
    fn new(x: usize, y: usize, alive: bool) -> Cell {
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
    pub fn update(&mut self) {
        let next = self.cells.clone();
        for (idx, cell) in next.iter().enumerate() {
            let mut alive_neighbors = 0;
            for offset in OFFSETS.iter() {
                let nx = cell.x as isize + offset.0;
                let ny = cell.y as isize + offset.1;
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
    pub fn width(&self) -> usize {
        self.width as usize
    }
    pub fn clone_cells(&self) -> Vec<Cell> {
        self.cells.clone()
    }
    pub fn set_cell_state(&mut self, cell: usize) {
        self.cells[cell].revive();
    }
    pub fn unset_cell_state(&mut self, cell: usize) {
        self.cells[cell].kill();
    }
    pub fn clear(&mut self) {
        for cell in &mut self.cells {
            cell.kill();
        }
    }
    pub fn new(width: usize, height: usize) -> Self {
        let mut cells: Vec<Cell> = Vec::new();
        for y in 0..height {
            for x in 0..width {
                cells.push(Cell::new(x, y, false))
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
