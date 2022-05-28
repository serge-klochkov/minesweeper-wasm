use rand::{Rng, thread_rng};

pub type Position = (usize, usize);

#[derive(Clone, Debug)]
pub enum Cell {
    // adjacent mines count
    Open(usize),
    // has mine?
    Flagged(bool),
    Closed(bool),
    RevealedMine,
}

#[derive(Debug)]
pub struct Minesweeper {
    pub width: usize,
    pub height: usize,
    pub mines_count: usize,
    pub cells: Vec<Cell>,
}

impl Minesweeper {
    pub fn new(width: usize, height: usize, mines_count: usize) -> Self {
        let cells = Minesweeper::init_cells(width, height, mines_count);
        Minesweeper {
            width,
            height,
            mines_count,
            cells,
        }
    }

    pub fn restart(&mut self) -> &mut Self {
        let cells = Minesweeper::init_cells(self.width, self.height, self.mines_count);
        self.cells = cells;
        self
    }

    /// returns true if flag was placed, false if removed
    pub fn toggle_flag(&mut self, pos: Position) -> bool {
        let idx = self.get_index(&pos);
        match self.cells[idx] {
            Cell::Open(_) => false,
            Cell::RevealedMine => false,
            Cell::Flagged(has_mine) => {
                self.cells[idx] = Cell::Closed(has_mine);
                false
            }
            Cell::Closed(has_mine) => {
                self.cells[idx] = Cell::Flagged(has_mine);
                true
            }
        }
    }

    /// returns true if the selected position contains a mine
    pub fn open(&mut self, position: Position) -> bool {
        let (idx, has_mine) = self.get_has_mine(&position);
        return if has_mine {
            for idx in 0..self.cells.len() {
                match &self.cells[idx] {
                    Cell::Flagged(has_mine) if *has_mine => self.cells[idx] = Cell::RevealedMine,
                    Cell::Closed(has_mine) if *has_mine => self.cells[idx] = Cell::RevealedMine,
                    _ => {}
                }
            }
            true // todo reveal
        } else {
            let mines_count = self.find_neighbors_mines_count(position);
            self.cells[idx] = Cell::Open(mines_count);
            false
        };
    }

    fn init_cells(width: usize, height: usize, mines_count: usize) -> Vec<Cell> {
        let mut cells = vec![Cell::Closed(false); width * height];
        let mut random = thread_rng();
        (0..mines_count).for_each(|_| {
            let x = random.gen_range(0..width);
            let y = random.gen_range(0..height);
            println!("Generated x {} y {}", x, y);
            cells[x * width + y] = Cell::Closed(true);
        });
        cells
    }

    fn get_index(&self, (x, y): &Position) -> usize {
        x * self.width + y
    }

    fn find_neighbors(&self, (x, y): Position) -> impl Iterator<Item=Position> + '_ {
        (x.max(1) - 1..=(x + 1).min(self.width - 1)).flat_map(move |i| {
            (y.max(1) - 1..=(y + 1).min(self.height - 1)).map(move |j| (i, j))
        }).filter(move |&pos| pos != (x, y))
    }

    fn find_neighbors_mines_count(&self, pos: Position) -> usize {
        self.find_neighbors(pos).filter(|pos| {
            self.get_has_mine(pos).1
        }).count()
    }

    fn get_has_mine(&self, position: &Position) -> (usize, bool) {
        let idx = self.get_index(position);
        let has_mine = match self.cells[idx] {
            Cell::Open(_) => false,
            Cell::RevealedMine => true,
            Cell::Flagged(has_mine) => has_mine,
            Cell::Closed(has_mine) => has_mine,
        };
        (idx, has_mine)
    }
}

#[cfg(test)]
mod tests {
    use crate::minesweeper::Minesweeper;

    #[test]
    fn test_new() {
        let width = 3;
        let height = 3;
        let mut ms = Minesweeper::new(width, height, 4);
        ms.open((1, 1));
        ms.toggle_flag((2, 2));
        println!("{:?}", ms);
    }
}