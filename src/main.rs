use clap::Parser;
use cli::Cli;

mod cli;
mod utils;

fn main() {
    let _args = Cli::parse();
}
