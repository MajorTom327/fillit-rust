use std::ops::Range;

use crate::tetrimino::Tetrimino;
use crate::fillit::Fillit;

pub struct Parser {
  file_content: String
}

const SIZE_INTERVAL: Range<u32> = 1..27;

impl Parser {
  pub fn new(file_content: String) -> Self {
    Self {
      file_content: file_content.clone()
    }
  }


  pub fn parse_content(&self) -> Fillit {
    let content = &self.file_content;

    let groups: Vec<&str> = content.split("\n\n")
      .into_iter().filter(|line| !line.is_empty()).collect::<Vec<&str>>();

    let len: u32 = groups.len().try_into().unwrap();

    if !SIZE_INTERVAL.contains(&(len)) {
      panic!("Seem like the len of items are invalid");
    }

    let mut i = 0;
    let mut tetriminos: Vec<Tetrimino> = Vec::new();
    for group in groups.iter() {

      let group = *group;
      let character: char = ('A' as u8 + i) as char;

      let tetrimino = Tetrimino::new(character, String::from(group));

      tetriminos.push(tetrimino);
      i += 1;
    }

    Fillit::new(tetriminos)

  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_parser() {
    let parser = Parser::new("##..\n##..\n....\n....".to_string());
    let fillit = parser.parse_content();
    assert_eq!(fillit.tetriminos.len(), 1);
    let tetrimino = &fillit.tetriminos[0];

    assert_eq!(tetrimino.char, 'A');
  }

  #[test]
  #[should_panic(expected = "Seem like the len of items are invalid")]
  fn test_invalid_numbers_of_tetriminos() {
    let parser = Parser::new("".to_string());
    let _fillit = parser.parse_content();
  }

  #[test]
  fn should_define_tetrimino() {
    let tetrimino = Tetrimino::new('A', String::from("##..\n##..\n....\n...."));
    assert_eq!(tetrimino.char, 'A');

    assert_eq!(tetrimino.shape, [[1,1,0,0], [1,1,0,0], [0,0,0,0], [0,0,0,0]]);
  }
}
