use std::fmt::Error;

pub const GAME_WIDTH: usize = 30;
pub const GAME_HEIGHT: usize = 30;

pub struct Game {
    grid: [[bool; GAME_WIDTH]; GAME_HEIGHT],
    pub ticks: u32,
}

// if cell is alive then it stays alive if it has 2 or 3 neighbours else, dies.
// if the cell is dead then it springs alive if it has three neighbours

impl Game {
    pub fn new() -> Game {
        Game {
            grid: [[false; GAME_WIDTH]; GAME_HEIGHT],
            ticks: 0,
        }
    }
    pub fn change(&mut self, point: (usize, usize)) -> Result<(), Error> {
        if valid_point(&(point.0 as isize, point.1 as isize)) {
            self.grid[point.1][point.0] ^= true;
            return Ok(());
        }
        Err(Error)
    }

    pub fn tick(&mut self) {
        self.ticks += 1;
        let mut changed: Vec<(usize, usize)> = Vec::new();
        for (row_index, row) in self.grid.iter().enumerate() {
            for (cell_index, cell) in row.iter().enumerate() {
                if *cell {
                    let number = self.check_neighbours((cell_index, row_index));
                    if !(number == 2 || number == 3) {
                        changed.push((cell_index, row_index));
                    }
                } else if self.check_neighbours((cell_index, row_index)) == 3 {
                    changed.push((cell_index, row_index));
                }
            }
        }
        for cell in changed.iter() {
            let _ = self.change(*cell);
        }
    }

    pub fn show(&self) -> [[bool; GAME_WIDTH]; GAME_HEIGHT] {
        self.grid
    }

    // x, y
    fn check_neighbours(&self, point: (usize, usize)) -> u8 {
        let cell_behind: isize = point.0 as isize - 1;
        let cell_ahead: isize = point.0 as isize + 1;
        let row_behind: isize = point.1 as isize - 1;
        let row_ahead: isize = point.1 as isize + 1;

        let mut n: u8 = 0;
        if self.check_state((cell_behind, row_behind)) {
            n += 1;
        }
        if self.check_state((cell_behind, point.1 as isize)) {
            n += 1;
        }
        if self.check_state((cell_behind, row_ahead)) {
            n += 1;
        }
        if self.check_state((point.0 as isize, row_behind)) {
            n += 1;
        }
        if self.check_state((point.0 as isize, row_ahead)) {
            n += 1;
        }
        if self.check_state((cell_ahead, row_behind)) {
            n += 1;
        }
        if self.check_state((cell_ahead, point.1 as isize)) {
            n += 1;
        }
        if self.check_state((cell_ahead, row_ahead)) {
            n += 1;
        }

        n
    }

    fn check_state(&self, point: (isize, isize)) -> bool {
        if valid_point(&point) {
            if self.grid[point.1 as usize][point.0 as usize] {
                return true;
            }
        }
        false
    }
}

fn valid_point(point: &(isize, isize)) -> bool {
    point.0 >= 0 && point.1 >= 0 && point.0 < GAME_WIDTH as isize && point.1 < GAME_HEIGHT as isize
}
