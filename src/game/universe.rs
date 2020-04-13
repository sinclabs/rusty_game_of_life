use rand::prelude::*;
use std::convert::TryInto;

#[derive(PartialEq)]
pub enum StartState {
    AllAlive,
    AllDead,
    RandomAlive,
}

pub struct Universe {
    pub cells: Vec<Vec<bool>>,
    pub size: usize,
}

/*
 * Create a new `Universe` for Game of Life.
 *
 * @param size: usize - Size of the universe. Both rows and columns use the same size.
 * @param start_state: StartState - The state of the cells when they are create. Ceels can either be
 *                                   alive or dead.
 *
 * @return Universe - newly constructed Universe object
 *
 */
pub fn new(size: usize, start_state: StartState) -> Universe {
    let start_bool;

    match start_state {
        StartState::AllAlive | StartState::RandomAlive => start_bool = true,
        StartState::AllDead => start_bool = false,
    }

    let mut universe = Universe {
        cells: vec![vec![start_bool; size]; size],
        size: size,
    };

    if start_state == StartState::RandomAlive {
        universe.randomize();
    }

    universe
}

fn get_random_bool() -> bool {
    rand::thread_rng().gen_range(1, 3) % 2 == 0
}

impl Universe {
    pub fn randomize(&mut self) {
        for row in 0..self.cells.len() {
            for col in 0..self.cells[row].len() {
                self.cells[row][col] = get_random_bool();
            }
        }
    }

    pub fn evolve(&mut self) {
        let mut next_generation = vec![vec![false; self.size]; self.size];

        for row in 0..self.cells.len() {
            for col in 0..self.cells[row].len() {
                let live_neighbours_count = self.live_neighbours_count(row, col);

                if *self.is_cell_alive(row, col)
                    && (live_neighbours_count == 2 || live_neighbours_count == 3)
                {
                    next_generation[row][col] = true;
                } else if !*self.is_cell_alive(row, col) && live_neighbours_count == 3 {
                    next_generation[row][col] = true;
                } else if *self.is_cell_alive(row, col) {
                    next_generation[row][col] = false;
                }
            }
        }

        self.cells = next_generation;
    }

    fn live_neighbours_count(&self, row: usize, col: usize) -> u8 {
        let mut count = 0;

        let row_overflow: i8 = row.try_into().unwrap();
        let col_overflow: i8 = col.try_into().unwrap();
        let size: i8 = self.size.try_into().unwrap();

        let mut update_count = |row: usize, col: usize| {
            if *self.is_cell_alive(row, col) {
                count += 1
            }
        };

        // Top left cell
        if (col_overflow - 1) > 0 && (row_overflow - 1) > 0 {
            update_count(row - 1, col - 1);
        }
        // Top middle cell
        if (row_overflow - 1) > 0 {
            update_count(row - 1, col);
        }
        // Top right cell
        if (col_overflow + 1) < size && (row_overflow - 1) > 0 {
            update_count(row - 1, col + 1);
        }
        // Middle left cel
        if (col_overflow - 1) > 0 {
            update_count(row, col - 1);
        }
        // Middle right cell
        if (col_overflow + 1) < size {
            update_count(row, col + 1);
        }
        // Bottom left cell
        if (col_overflow - 1) > 0 && (row_overflow + 1) < size {
            update_count(row + 1, col - 1);
        }
        // Bottom middle cell
        if (row_overflow + 1) < size {
            update_count(row + 1, col);
        }
        // Bottom right cell
        if (col_overflow + 1) < size && (row_overflow + 1) < size {
            update_count(row + 1, col + 1);
        }

        count
    }

    fn is_cell_alive(&self, row: usize, col: usize) -> &bool {
        &self.cells[row][col]
    }
}
