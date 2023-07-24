use crate::grid::Grid;
use crate::tetrimino::Tetrimino;

pub struct Fillit {
  pub tetriminos: Vec<Tetrimino>,
  pub grid: Grid
}

impl Fillit {
  pub fn new(tetriminos: Vec<Tetrimino>) -> Self {
    // let min_size: u32 = (tetriminos.len() * 2).try_into().unwrap();

    let mut size: u32 = 2;
    let n: u32 = tetriminos.len() as u32 * 4;
    while size * size < n {
      size += 1;
    }

    let min_size = size;

    Self {
      tetriminos,
      grid: Grid::new(min_size, min_size)
    }
  }

  pub fn solve(&mut self) {
    let mut solved: bool = true;
    for tetrimino in self.tetriminos.iter() {
      let mut placed = false;
      let height = if self.grid.height < tetrimino.height {
        tetrimino.height
      } else {
        self.grid.height - tetrimino.height + 1
      };

      for y in 0..height {
        let width = if self.grid.width < tetrimino.width {
          tetrimino.width
        } else {
          self.grid.width - tetrimino.width + 1
        };

        for x in 0..width {
          let res = self.grid.insert(x, y, tetrimino);
          match res {
            Ok(_) => {
              placed = true;
              break
            }
            Err(_) => { continue }
          }
        }
        if placed {
          break;
        }

      }
      if !placed {
        solved = false;
        break;
      }
    }

    if solved {
      return;
    }

    self.grid.grow(1);
    self.solve();
  }

  pub fn print(&self) {
    self.grid.print();
  }
}
