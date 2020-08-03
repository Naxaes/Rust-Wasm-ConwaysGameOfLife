extern crate js_sys;
extern crate fixedbitset;
extern crate web_sys;
extern crate itertools;

pub use crate::bindings::*;

use itertools::Itertools;
use wasm_bindgen::prelude::*;
use fixedbitset::FixedBitSet;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

type CellArray = FixedBitSet;
const CELL_DEAD:  bool = false;
const CELL_ALIVE: bool = true;

#[wasm_bindgen]
pub struct Cell(pub u32, pub u32);

#[wasm_bindgen]
pub struct Universe {
    width:  u32,
    height: u32,
    cells:  CellArray,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        utils::set_panic_hook();
        log!("Starting universe {}", 1);

        let width  = 128;
        let height = 128;

        let mut cells = FixedBitSet::with_capacity(width * height);
        for i in 0..width * height {
            cells.set(i, js_sys::Math::random() < 0.5);
        }

        Universe { width: width as u32, height: height as u32, cells }
    }

    pub fn tick(&mut self) {
        let _timer = utils::Timer::new("Universe::tick");

        let mut next = {
            let _timer = utils::Timer::new("Allocate next cells");
            self.cells.clone()
        };

        {
            let _timer = utils::Timer::new("New generation");
            for row in 0..self.height {
                for col in 0..self.width {
                    let idx = self.get_index(row, col);
                    let cell = self.cells[idx];
                    let live_neighbors = self.live_neighbor_count(row, col);

                    let next_cell = match (cell, live_neighbors) {
                        // Rule 1: Any live cell with fewer than two live neighbours
                        // dies, as if caused by underpopulation.
                        (CELL_ALIVE, x) if x < 2 => CELL_DEAD,
                        // Rule 2: Any live cell with two or three live neighbours
                        // lives on to the next generation.
                        (CELL_ALIVE, 2) | (CELL_ALIVE, 3) => CELL_ALIVE,
                        // Rule 3: Any live cell with more than three live
                        // neighbours dies, as if by overpopulation.
                        (CELL_ALIVE, x) if x > 3 => CELL_DEAD,
                        // Rule 4: Any dead cell with exactly three live neighbours
                        // becomes a live cell, as if by reproduction.
                        (CELL_DEAD, 3) => CELL_ALIVE,
                        // All other cells remain in the same state.
                        (otherwise, _) => otherwise,
                    };

                    next.set(idx, next_cell);
                }
            }
        }

        let _timer = utils::Timer::new("Copy cells");
        self.cells = next;
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    /// Set the width of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = FixedBitSet::with_capacity((self.width * self.height) as usize);
    }

    /// Set the height of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = FixedBitSet::with_capacity((self.width * self.height) as usize);
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells.set(idx, !self.cells[idx]);
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    pub fn clear(&mut self) {
        for row in 0..self.height {
            for col in 0..self.width {
                let index = self.get_index(row, col);
                self.cells.set(index, false);
            }
        }
    }

    pub fn reset(&mut self) {
        for i in 0..self.width * self.height {
            self.cells.set(i as usize, js_sys::Math::random() < 0.5);
        }
    }

    pub fn set_cells(&mut self, cells: &[u32]) {
        if cells.len() % 2 == 1 {
            panic!("Argument 'cells' must be pairs of (x, y).");
        }
        for (row, col) in cells.iter().tuples() {
            if *row <= self.height && *col <= self.width {  // x, y can't be less than 0 as they're u32.
                let index = self.get_index(*row, *col);
                self.cells.set(index, true);
            }
        }
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row    + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let index = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[index] as u8;
            }
        }
        count
    }
}


// Rust-generated WebAssembly functions cannot return borrowed references.
impl Universe {
    /// Get the dead and alive values of the entire universe.
    pub fn get_cells(&self) -> &CellArray {
        &self.cells
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells_(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells.set(idx, CELL_ALIVE);
        }
    }
}