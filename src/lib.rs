mod utils;

use wasm_bindgen::prelude::*;

extern crate js_sys;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width -1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
    
    /// get the dead and alive values of the entire universe
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row,col);
            self.cells[idx] = Cell::Alive;
        }
    }
}

/// public methods exported to javascript
#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.width {
            for col in 0..self.height {
                let idx = self.get_index(row,col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row,col);

                println!(
                    "cell[{}, {}] is initially {:?} and has {} live neighbours",
                    row,
                    col,
                    cell,
                    live_neighbors
                    );

                let next_cell = match(cell, live_neighbors) {
                    // rule 1: Any live cell with fewer than 2 live neighbrs
                    // dies as, as if by underpopulation
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // rule 2: any live cell with two or three gets to live
                    // on the to the next generation
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // rule 3: any live cell with more than 3 neighbours
                    // dies as if by overpopulation
                    (Cell::Alive,x) if x>3 => Cell::Dead,
                    // rule 4: any dead cell with exactly 3 live neighbours
                    // becomes a live cell as if by reproduction
                    (Cell::Dead, 3) => Cell::Alive,
                    // all other cells retain their states
                    (otherwise, _) => otherwise,
                };
                
                println!("  it becomes {:?}", next_cell);
                next[idx] = next_cell;
            }
        }
        self.cells = next;
    }

    pub fn new() -> Universe {
        utils::set_panic_hook();
        let width = 8;
        let height = 8;
         
        let _spaceship = [1,4,width,2*width,2*width+4,width*3,
                        width*3+1,width*3 + 2, width*3 + 3];
         
        let _oscillator = [width*7 + width/2 - 1, width*7 +width/2, 
                          width*7 +width/2 + 1];
        
        let cells = (0..width * height)
            .map(|_i| {
                if js_sys::Math::random() < 0.4 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
     }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    /// set the width of the universe
    ///
    /// Resets all the cells to the dead state
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height)
            .map(|_i| Cell::Dead).collect();
    } 
    
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..self.width * height)
            .map(|_i| Cell::Dead).collect();
    }

}

use std::fmt;

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead{ '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
                }
                write!(f, "\n")?;
            }

            Ok(())
        }
}




