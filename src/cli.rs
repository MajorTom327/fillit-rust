use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short('i'), long)]
    pub input: std::path::PathBuf
}
