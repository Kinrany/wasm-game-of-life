use wasm_bindgen::prelude::*;

use {
  crate::{cell::Cell, rules::rules},
  js_sys::Math,
  std::fmt,
};

#[wasm_bindgen]
pub struct Universe {
  width: u32,
  height: u32,
  cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
  pub fn width(&self) -> u32 {
    self.width
  }

  pub fn height(&self) -> u32 {
    self.height
  }

  pub fn cells_ptr(&self) -> *const Cell {
    self.cells.as_ptr()
  }

  pub fn set_cell(&mut self, row: u32, column: u32, value: Cell) {
    let idx = self.get_cell_index(row, column);
    self.cells[idx] = value;
  }

  pub fn new(width: u32, height: u32) -> Universe {
    let cells = vec![Cell::Dead; (width * height) as usize];

    Universe {
      width,
      height,
      cells,
    }
  }

  pub fn randomize(&mut self, density: f64) {
    // use Math.random() to initialize the cells
    let len = (self.width * self.height) as usize;
    for i in 0..len {
      self.cells[i] = match Math::random() {
        x if x < density => Cell::Alive,
        _x => Cell::Dead,
      }
    }
  }

  pub fn tick(&mut self) {
    let mut next_cells = self.cells.clone();

    for row in 0..self.height {
      for column in 0..self.width {
        let idx = self.get_cell_index(row, column);
        let cell = self.cells[idx];
        let live_neighbor_count = self.alive_neighbor_count(row, column);

        let next_cell = rules(&cell, live_neighbor_count);
        next_cells[idx] = next_cell;
      }
    }

    self.cells = next_cells;
  }

  fn get_cell_index(&self, row: u32, column: u32) -> usize {
    (row * self.width + column) as usize
  }

  fn alive_neighbor_count(&self, row: u32, column: u32) -> u8 {
    let mut count = 0;
    for delta_row in [self.height - 1, 0, 1].iter().cloned() {
      for delta_column in [self.width - 1, 0, 1].iter().cloned() {
        if delta_row == 0 && delta_column == 0 {
          continue;
        }

        let neighbor_row = (row + delta_row) % self.height;
        let neighbor_column = (column + delta_column) % self.width;
        let idx = self.get_cell_index(neighbor_row, neighbor_column);
        count += self.cells[idx] as u8;
      }
    }
    count
  }
}

impl Universe {
  pub fn get_cells(&self) -> &[Cell] {
    &self.cells
  }

  pub fn set_cells(&mut self, cells: &[(u32, u32)], value: Cell) {
    for (row, col) in cells.iter().cloned() {
      let idx = self.get_cell_index(row, col);
      self.cells[idx] = value;
    }
  }
}

impl fmt::Display for Universe {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for line in self.cells.as_slice().chunks(self.width() as usize) {
      for &cell in line {
        let symbol = match cell {
          Cell::Dead => '◻',
          Cell::Alive => '◼',
        };
        write!(f, "{}", symbol)?;
      }
      write!(f, "\n")?;
    }

    Ok(())
  }
}
