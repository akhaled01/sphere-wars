use clap::Parser;

mod cli;
mod utils;

use cli::Args;

fn main() {
    let args = Args::parse();
    utils::print_info(args);
}
