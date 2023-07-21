use crate::tetrimino::Tetrimino;

pub struct Grid {
  pub width: u32,
  pub height: u32,
  pub cells: Vec<char>,
}

impl Grid {
  pub fn new(width: u32, height: u32) -> Grid {
    let mut cells = Vec::new();
    for _ in 0..width * height {
      cells.push(' ');
    }
    Grid {
      width,
      height,
      cells,
    }
  }

  pub fn get(&self, x: u32, y: u32) -> char {
    let cell_index = (y * self.width + x) as usize;
    if cell_index >= self.cells.len() {
      return '?';
    }
    self.cells[cell_index]
  }

  pub fn set(&mut self, x: u32, y: u32, c: char) {
    let position = y * self.width + x;
    if position > self.cells.len() as u32 {
      return;
    }

    if self.get(x, y) != ' ' {
      return;
    }

    if x < self.width && y < self.height {
      self.cells[(y * self.width + x) as usize] = c;
    }
  }

  /// Extend the grid and the bottom of size lines and columns
  pub fn grow(&mut self, size: u32) {
    self.width += size;
    self.height += size;

    let mut cells = Vec::new();
    for _ in 0..self.width * self.height {
      cells.push(' ');
    }

    self.cells = cells;
  }


  pub fn can_insert(&self, x: u32, y: u32, tetrimino: &Tetrimino) -> bool {
    let mut i = 0;

    for line in tetrimino.shape.iter() {
      let mut j = 0;
      for cell in line.iter() {
        let pos_x = x + <usize as TryInto<u32>>::try_into(j).unwrap();
        let pos_y = y + <usize as TryInto<u32>>::try_into(i).unwrap();

        if *cell == 1 && self.get(pos_x, pos_y) != ' ' {
          return false;
        }
        j += 1;
      }
      i += 1;
    }

    true
  }

  pub fn insert(&mut self, x: u32, y: u32, tetrimino: &Tetrimino) -> Result<bool, &'static str> {
    let mut i = 0;

    let can_insert = self.can_insert(x, y, tetrimino);
    if !can_insert {
      return Err("Cannot place the tetrimino");
    }

    for line in tetrimino.shape.iter() {
      let mut j = 0;
      for _cell in line.iter() {
        let tetri_cell = tetrimino.shape[i][j];
        let pos_x = x + <usize as TryInto<u32>>::try_into(j).unwrap();
        let pos_y = y + <usize as TryInto<u32>>::try_into(i).unwrap();

        if tetri_cell == 1 && self.get(pos_x, pos_y) == ' ' {
          self.set(pos_x, pos_y, tetrimino.char);
        }
        j += 1;
      }
      i += 1;
    }
    Ok(true)

  }

  pub fn print(&self) {
    for y in 0..self.height {
      for x in 0..self.width {
        let cell = self.get(x, y);

        match cell {
        ' ' => print!("."),
        _ => print!("{}", cell),
      }
      }
      println!();
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::tetrimino;

use super::*;

  #[test]
  fn test_grid() {
    let grid = Grid::new(10, 20);
    assert_eq!(grid.get(0, 0), ' ');
  }

  #[test]
  fn test_grid_set() {
    let mut grid = Grid::new(10, 20);
    grid.set(0, 0, 'x');
    assert_eq!(grid.get(0, 0), 'x');
  }

  #[test]
  fn test_grid_set_out_of_bounds() {
    let mut grid = Grid::new(10, 20);
    grid.set(10, 20, 'x');
    for cell in grid.cells {
      assert_eq!(cell, ' ');
    }
  }


  #[test]
  fn test_already_set_cell() {
    let mut grid = Grid::new(10, 20);
    grid.set(0, 0, 'x');
    grid.set(0, 0, 'y');
    assert_eq!(grid.get(0, 0), 'x');
  }

  #[test]
  fn test_insert_tetrimino() {
    let mut grid = Grid::new(10, 20);
    let tetrimino = tetrimino::Tetrimino::from_shape('A', [[1,1,0,0], [1,1,0,0], [0,0,0,0], [0,0,0,0]]);

    grid.insert(0, 0, &tetrimino);
    assert_eq!(grid.get(0, 0), 'A');
    assert_eq!(grid.get(1, 0), 'A');
    assert_eq!(grid.get(0, 1), 'A');
    assert_eq!(grid.get(1, 1), 'A');
  }

  #[test]
  fn test_insert_over_already_inserted() {
    let mut grid = Grid::new(10, 20);
    let tetrimino = tetrimino::Tetrimino::from_shape('A', [[1,1,0,0], [1,1,0,0], [0,0,0,0], [0,0,0,0]]);

    grid.insert(0, 0, &tetrimino).unwrap();
    let tetrimino = tetrimino::Tetrimino::from_shape('B', [[1,1,0,0], [1,1,0,0], [0,0,0,0], [0,0,0,0]]);

    let result = grid.insert(0, 0, &tetrimino);
    assert_eq!(result, Err("Cannot place the tetrimino"));
  }

  #[test]
  fn test_grow_grid() {

    let mut grid = Grid::new(10, 20);
    grid.grow(1);

    assert_eq!(grid.width, 11);
    assert_eq!(grid.height, 21);
    assert_eq!(grid.cells.len(), 11 * 21);
  }
}
