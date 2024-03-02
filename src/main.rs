use std::env::args;
use std::str::FromStr;

use wc::{bytes_count, lines_count, words_count};

pub enum Mode {
    Words,
    Lines,
    Bytes,
}

impl FromStr for Mode {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-w" | "--words" => Ok(Mode::Words),
            "-l" | "--lines" => Ok(Mode::Lines),
            "-b" | "--bytes" => Ok(Mode::Bytes),
            _ => Err("Unknown mode, use -w|--words or -l|--lines"),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <file_path> (-w | -l)", args[0]);
        std::process::exit(1);
    }

    let path = &args[1];
    let mode = Mode::from_str(&args[2])?;

    let result = match mode {
        Mode::Words => words_count(&path)?,
        Mode::Lines => lines_count(&path)?,
        Mode::Bytes => bytes_count(&path)?,
    };

    println!("{result} {}", path);

    Ok(())
}
