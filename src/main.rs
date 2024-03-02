use std::str::FromStr;
use std::{env, path::Path};

use wc::{line_count, words_count};

pub enum Mode {
    Words,
    Lines,
}

impl FromStr for Mode {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-w" | "--words" => Ok(Mode::Words),
            "-l" | "--lines" => Ok(Mode::Lines),
            _ => Err("Unknown mode, use -w|--words or -l|--lines"),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <file_path> (-w | -l)", args[0]);
        std::process::exit(1);
    }

    let path_str = &args[1];
    let path = Path::new(path_str).canonicalize()?;
    let mode = Mode::from_str(&args[2])?;

    let result = match mode {
        Mode::Words => words_count(&path)?,
        Mode::Lines => line_count(&path)?,
    };

    println!("{result} {}", path.display());

    Ok(())
}
