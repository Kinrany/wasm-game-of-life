use crate::cell::Cell;

pub fn rules(cell: &Cell, alive_neighbors: u8) -> Cell {
  match (cell, alive_neighbors) {
    // Rule 1: Any live cell with fewer than two live neighbours
    // dies, as if caused by underpopulation.
    (Cell::Alive, x) if x < 2 => Cell::Dead,
    // Rule 2: Any live cell with two or three live neighbours
    // lives on to the next generation.
    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
    // Rule 3: Any live cell with more than three live
    // neighbours dies, as if by overpopulation.
    (Cell::Alive, x) if x > 3 => Cell::Dead,
    // Rule 4: Any dead cell with exactly three live neighbours
    // becomes a live cell, as if by reproduction.
    (Cell::Dead, 3) => Cell::Alive,
    // All other cells remain in the same state.
    (cell, _) => cell.clone(),
  }
}
