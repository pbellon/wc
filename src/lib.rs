mod models;
mod parser;

use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

use models::Counts;

pub use crate::models::{Metric, WcLineResult};

pub use crate::parser::CliParser;

pub fn count_file(path: &str) -> io::Result<Counts> {
    let path = Path::new(path).canonicalize()?;
    let file = File::open(path)?;

    let bytes = file.metadata()?.len();
    let reader = BufReader::new(file);

    let mut lines: u64 = 0;
    let mut words = 0;

    for line in reader.lines().map_while(Result::ok) {
        lines += 1;
        words += line.split_ascii_whitespace().count() as u64;
    }

    Ok(Counts(lines, words, bytes))
}

pub fn process_files(paths: &Vec<String>, metrics: &[Metric]) -> io::Result<Vec<WcLineResult>> {
    let mut total_counts = Counts(0, 0, 0);
    let mut results: Vec<WcLineResult> = Vec::new();

    for path in paths {
        let file_counts = count_file(path)?;
        total_counts = total_counts + file_counts;
        results.push(WcLineResult::from(&file_counts, path, metrics));
    }

    // only happen global total if command executed on more than 1 file
    if results.len() > 1 {
        results.push(WcLineResult::from(&total_counts, "total", metrics));
    }

    Ok(results)
}
