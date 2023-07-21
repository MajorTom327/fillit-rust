use clap::Parser;

mod grid;
mod tetrimino;
mod cli;
mod parser;
mod fillit;


pub fn run() {
  let args = cli::Cli::parse();

  let parser = parser::Parser::new(args.input);
  let mut fillit = parser.parse();

  fillit.solve();
  fillit.print();
}
