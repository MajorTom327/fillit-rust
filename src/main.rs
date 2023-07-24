use std::fs;

use clap::Parser;

use fillit::run;
use fillit::cli;

fn main() {

  let args = cli::Cli::parse();

  let file = fs::read_to_string(args.input);

  let file_content = match file {
    Ok(file) => file,
    Err(err) => panic!("Something is wrong, I can fillit...\n{}", err)
  };


  run(file_content);
}
