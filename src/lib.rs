mod grid;
mod tetrimino;
pub mod cli;
mod parser;
mod fillit;


pub fn run(content: String) {
  let fillit = solve_fillit(String::from(content));
  fillit.print();
}

pub fn solve_fillit(content: String) -> fillit::Fillit {
  let parser = parser::Parser::new(content);
  let mut fillit = parser.parse_content();

  fillit.solve();

  fillit.grid.cells = fillit.grid.cells.iter().map(|c| match c {
    ' ' => '.',
    _ => *c
  }).collect::<Vec<char>>();
  fillit
}
