// use std::env::args;
use std::io;

use clap::Parser;
use wc::{print_results, run, CliParser};

fn main() -> Result<(), io::Error> {
    let args = CliParser::parse();
    let results = run(&args)?;
    print_results(&results);

    Ok(())
}
