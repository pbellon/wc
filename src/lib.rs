mod models;
mod parser;

use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

pub use crate::models::{Metric, WcLineResult};

pub use crate::parser::CliParser;

pub fn count_file(path: &str) -> io::Result<(u64, u64, u64)> {
    let path = Path::new(path).canonicalize()?;
    let file = File::open(path)?;

    let bytes = file.metadata()?.len();
    let reader = BufReader::new(file);

    let mut lines = 0;
    let mut words = 0;

    for line in reader.lines().map_while(Result::ok) {
        lines += 1;
        words += line.split_ascii_whitespace().count();
    }

    Ok((lines as u64, words as u64, bytes))
}

pub fn process_files(paths: &Vec<String>, metrics: &[Metric]) -> io::Result<Vec<WcLineResult>> {
    let mut total_counts: (u64, u64, u64) = (0, 0, 0);
    let mut results: Vec<WcLineResult> = Vec::new();

    for path in paths {
        let file_counts = count_file(path)?;
        // Happen current counts to global res
        total_counts.0 += file_counts.0;
        total_counts.1 += file_counts.1;
        total_counts.2 += file_counts.2;
        results.push(WcLineResult::from(&file_counts, path, metrics));
    }

    // only happen global total if command executed on more than 1 file
    if results.len() > 1 {
        results.push(WcLineResult::from(&total_counts, "total", metrics));
    }

    Ok(results)
}
