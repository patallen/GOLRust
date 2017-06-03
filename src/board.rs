const OFFSETS: [(isize, isize); 8] = [
    (0, -1), (1, -1), (1, 0), (1, 1),
    (0, 1), (-1, 1), (-1, 0), (-1, -1),
];

#[derive(Clone, Debug)]
enum CellState {
    Alive,
    Dead,
}

#[derive(Clone, Debug)]
pub struct Cell { state: CellState }
impl Cell {
    fn new(alive: bool) -> Cell {
        let state = match alive {
            true => CellState::Alive,
            false => CellState::Dead,
        };
        Cell { state: state }
    }
    fn kill(&mut self) {
        self.state = CellState::Dead;
    }
    fn revive(&mut self) {
        self.state = CellState::Alive;
    }
    pub fn is_alive(&self) -> bool {
        match self.state {
            CellState::Alive => true,
            _ => false
        }
    }
}

pub struct Board {
    cells: Vec<Vec<Cell>>,
    width: isize,
    height: isize,
}

impl Board {
    pub fn update(&mut self) {
        let prev = self.cells.clone();
        for (ridx, row) in prev.iter().enumerate() {
            for (cidx, temp) in row.iter().enumerate() {
                let mut alive_neighbors = 0;
                for offset in OFFSETS.iter() {
                    let nrow = ridx as isize + offset.1;
                    let ncol = cidx as isize + offset.0;
                    if nrow >= 0 && nrow < self.height && ncol >= 0 && ncol < self.width {
                        if prev[nrow as usize][ncol as usize].is_alive() { alive_neighbors += 1; }
                    }
                }

                let mut cell = &mut self.cells[ridx][cidx];
                match (temp.is_alive(), alive_neighbors) {
                    (true, 2) | (true, 3) => {}
                    (false, 3) => { cell.revive() },
                    _ => { cell.kill() }
                };
            }
        }
    }
    pub fn clone_cells(&self) -> Vec<Vec<Cell>> {
        self.cells.clone()
    }
    pub fn set_cell_state(&mut self, x: usize, y: usize) {
        self.cells[y][x].revive();
    }
    pub fn unset_cell_state(&mut self, x: usize, y: usize) {
        self.cells[y][x].kill();
    }
    pub fn clear(&mut self) {
        for row in &mut self.cells {
            for cell in row.iter_mut() {
                cell.kill();
            }
        }
    }
    pub fn new(width: usize, height: usize) -> Self {
        let mut cells: Vec<Vec<Cell>> = Vec::new();
        for _ in 0..height {
            let mut row = Vec::new();
            for _ in 0..width {
                row.push(Cell::new(false))
            }
            cells.push(row);
        }
        Board {
            height: height as isize,
            width: width as isize,
            cells: cells,
        }
    }
}
