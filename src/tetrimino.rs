use std::fmt::Display;
use regex::Regex;
use once_cell::sync::Lazy;

pub struct Tetrimino {
  pub char: char,
  pub shape: [[u8; 4]; 4],

  pub width: u32,
  pub height: u32
}

const LINE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[#.]{4}$").unwrap());

impl Tetrimino {
  pub fn new(char: char, shape: String) -> Tetrimino {

    let lines = shape.split("\n").collect::<Vec<&str>>()
      .into_iter().filter(|line| !line.is_empty()).collect::<Vec<&str>>();

    if lines.len() != 4 {
      panic!("Invalid tetrimino shape");
    }

    for line in lines.iter() {
      let line = *line;
      if !LINE_REGEX.is_match(line) {
        panic!("Invalid tetrimino shape");
      }
    }

    let mut nb_bloc = 0;

    for line in lines.iter() {
      let line = *line;
      for c in line.chars() {
        if c == '#' {
          nb_bloc += 1;
        }
      }
    }

    if nb_bloc != 4 {
      panic!("Invalid tetrimino shape");
    }

    let mut shape: [[u8; 4]; 4] = [[0; 4]; 4];

    let mut i = 0;
    for line in lines.iter() {
      let line = *line;
      let mut j = 0;
      for c in line.chars() {
        if c == '#' {
          shape[i][j] = 1;
        }
        j += 1;
      }
      i += 1;
    }


    shape = Tetrimino::move_to_top(shape);
    shape = Tetrimino::move_to_left(shape);

    let (width, height) = Tetrimino::get_size(shape);

    Tetrimino {
      char,
      shape,
      width,
      height
    }
  }

  fn get_size(shape: [[u8; 4]; 4]) -> (u32, u32) {
    let mut width = 0;
    let mut height = 0;

    for line in shape.iter() {
      let line = *line;
      let mut i = 0;
      for c in line.iter() {
        if *c == 1 {
          width = i + 1;
        }
        i += 1;
      }
    }

    for col in 0..4 {
      let mut i = 0;
      for line in 0..4 {
        if shape[line][col] == 1 {
          height = i + 1;
        }
        i += 1;
      }
    }

    return (width, height);
  }
  /// Test utility to not have to create a string
  #[cfg(test)]
  pub fn from_shape(char: char, shape: [[u8; 4]; 4]) -> Tetrimino {
    let mut shape = shape;

    shape = Tetrimino::move_to_top(shape);
    shape = Tetrimino::move_to_left(shape);

    let (width, height) = Tetrimino::get_size(shape);

    Tetrimino {
      char,
      shape,
      width,
      height
    }
  }

  fn move_to_top(shape: [[u8; 4]; 4]) -> [[u8; 4]; 4] {
    let mut i = 0;
    let mut shape = shape;

    for line in shape.iter() {
      let line = *line;
      if line[0] == 0 && line[1] == 0 && line[2] == 0 && line[3] == 0 {
        i += 1;
      } else {
        break;
      }
    }

    if i == 0 {
      return shape;
    }

    for line in i..4 {
      shape[line - i] = shape[line];
      shape[line] = [0, 0, 0, 0];
    }

    shape
  }

  fn move_to_left(shape: [[u8; 4]; 4]) -> [[u8; 4]; 4] {

    let mut empty_column = 0;

    for col in 0..4 {
      let mut empty = true;
      for line in 0..4 {
        if shape[line][col] == 1 {
          empty = false;
          break;
        }
      }
      if empty {
        empty_column += 1;
      } else {
        break;
      }
    }

    if empty_column == 0 {
      return shape;
    }

    let mut shape = shape;

    for col in empty_column..4 {
      for line in 0..4 {
        shape[line][col - empty_column] = shape[line][col];
        shape[line][col] = 0;
      }
    }

    shape
  }

}

impl Display for Tetrimino {

  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for line in self.shape.iter() {
      let line = *line;
      for c in line.iter() {
        if *c == 1 {
          write!(f, "{}", self.char)?;
        } else {
          write!(f, ".")?;
        }
      }
      write!(f, "\n")?;
    }
    Ok(())
  }

}


#[cfg(test)]
mod tests {
    use super::Tetrimino;

  #[test]
  #[should_panic(expected = "Invalid tetrimino shape")]
  fn should_have_four_lines() {
    let shape = String::from(
      "#...\n##..\n.#.."
    );
    let _tetrimino = super::Tetrimino::new('A', shape);
  }


  #[test]
  #[should_panic(expected = "Invalid tetrimino shape")]
  fn should_have_only_four_block() {
    let shape = String::from(
      "#...\n##..\n.##.\n....\n"
    );
    let _tetrimino = super::Tetrimino::new('A', shape);
  }

  #[test]
  #[should_panic(expected = "Invalid tetrimino shape")]
  fn should_have_line_correctly_defined() {
    let shape = String::from(
      "##..\n##..\n..\n....\n"
    );
    let _tetrimino = super::Tetrimino::new('A', shape);
  }


  #[test]
  fn should_create_tetrimino_top_left() {
    let shape = String::from(
      "....\n....\n..##\n..##\n"
    );
    let tetrimino = super::Tetrimino::new('A', shape);

    assert_eq!(tetrimino.shape, [
      [1,1,0,0],
      [1,1,0,0],
      [0,0,0,0],
      [0,0,0,0],
    ])
  }


  #[test]
  fn should_move_tetrimino_top() {
    let shape = [[0,0,0,0],[0,0,0,0], [0, 0, 1, 1], [0, 0, 1, 1]];
    let tetrimino = super::Tetrimino::move_to_top(shape);

    assert_eq!(tetrimino, [[0, 0, 1, 1], [0, 0, 1, 1], [0, 0, 0, 0], [0, 0, 0, 0]]);
  }

  #[test]
  fn should_not_move_tetrimino_top_if_no_space() {
    let shape = [[0,0,0,1],[0,0,0,1], [0, 0, 0, 1], [0, 0, 0, 1]];
    let tetrimino = super::Tetrimino::move_to_top(shape);

    assert_eq!(tetrimino, [[0, 0, 0, 1], [0, 0, 0, 1], [0, 0, 0, 1], [0, 0, 0, 1]]);
  }
  #[test]
  fn should_not_move_tetrimino_top_if_no_space_2() {
    let shape = [[0,0,0,1],[0,0,1,1], [0, 0, 1, 0], [0, 0, 0, 0]];
    let tetrimino = super::Tetrimino::move_to_top(shape);

    assert_eq!(tetrimino, [[0, 0, 0, 1], [0, 0, 1, 1], [0, 0, 1, 0], [0, 0, 0, 0]]);
  }

  #[test]
  fn should_move_tetrimino_left() {
    let shape = [[0,0,0,0],[0,0,0,0], [0, 0, 1, 1], [0, 0, 1, 1]];
    let tetrimino = super::Tetrimino::move_to_left(shape);

    assert_eq!(tetrimino, [[0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 0, 0], [1, 1, 0, 0]]);
  }

  #[test]
  fn should_get_size() {
    let shape = [[1,1,0,0], [1,1,0,0], [0,0,0,0], [0,0,0,0]];

    let (width, height) = Tetrimino::get_size(shape);

    assert_eq!(width, 2);
    assert_eq!(height, 2);
  }
}
