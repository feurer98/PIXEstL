//! PIXEstL command-line application

use clap::Parser;
use pixestl::cli::Cli;
use std::process;

fn main() {
    let cli = Cli::parse();

    if let Err(e) = cli.run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
