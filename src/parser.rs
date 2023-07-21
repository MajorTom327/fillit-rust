use std::{
  fs::File,
  io::Read, ops::Range,
};

use crate::tetrimino::Tetrimino;
use crate::fillit::Fillit;

pub struct Parser {
  file: std::path::PathBuf
}

const SIZE_INTERVAL: Range<u32> = 1..27;

impl Parser {

  pub fn new(file: std::path::PathBuf) -> Self {
    Self {
      file
    }
  }

  fn read_file(&self) -> Fillit {
    let file = File::open(self.file.to_string_lossy().to_string());

    let mut file = match file {
      Ok(file) => file,
      Err(err) => panic!("Something is wrong, I can fillit...\n{}", err)
    };

    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    Parser::parse_content(content)
  }

  fn parse_content(content: String) -> Fillit {
    let groups: Vec<&str> = content.split("\n\n")
      .into_iter().filter(|line| !line.is_empty()).collect::<Vec<&str>>();

    let len: u32 = groups.len().try_into().unwrap();

    dbg!(&groups);

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

  pub fn parse(&self) -> Fillit {
    self.read_file()
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_parser() {
    let fillit = Parser::parse_content("##..\n##..\n....\n....".to_string());
    assert_eq!(fillit.tetriminos.len(), 1);
    let tetrimino = &fillit.tetriminos[0];

    assert_eq!(tetrimino.char, 'A');
  }

  #[test]
  #[should_panic(expected = "Seem like the len of items are invalid")]
  fn test_invalid_numbers_of_tetriminos() {
    Parser::parse_content("".to_string());
  }

  #[test]
  fn should_define_tetrimino() {
    let tetrimino = Tetrimino::new('A', String::from("##..\n##..\n....\n...."));
    assert_eq!(tetrimino.char, 'A');

    assert_eq!(tetrimino.shape, [[1,1,0,0], [1,1,0,0], [0,0,0,0], [0,0,0,0]]);
  }
}
