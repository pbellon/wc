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

pub fn process_file(path: &str) -> io::Result<Counts> {
    let path = Path::new(path).canonicalize()?;
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    process_lines(reader)
}

pub fn process_stdin() -> io::Result<Counts> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    process_lines(reader)
}

pub fn process_lines<R: BufRead>(reader: R) -> io::Result<Counts> {
    let mut lines: u64 = 0;
    let mut words: u64 = 0;
    let mut bytes: u64 = 0;

    for line_result in reader.lines() {
        let line = line_result?;
        lines += 1;
        words += line.split_ascii_whitespace().count() as u64;
        bytes += line.len() as u64 + 1;
    }

    if bytes > 0 {
        bytes -= 1;
    }

    Ok(Counts(lines, words, bytes))
}

pub fn process_files(paths: &Vec<String>, metrics: &[Metric]) -> io::Result<Vec<WcLineResult>> {
    let mut total_counts = Counts(0, 0, 0);
    let mut results: Vec<WcLineResult> = Vec::new();

    if paths.is_empty() || (paths.len() == 1 && paths[0] == "-") {
        let stdin_counts = process_stdin()?;
        total_counts = total_counts + stdin_counts;
        results.push(WcLineResult::from(&stdin_counts, "", metrics));
    } else {
        for path in paths {
            let file_counts = process_file(path)?;
            total_counts = total_counts + file_counts;
            results.push(WcLineResult::from(&file_counts, path, metrics));
        }
    }
    // only happen global total if command executed on more than 1 file
    if results.len() > 1 {
        results.push(WcLineResult::from(&total_counts, "total", metrics));
    }

    Ok(results)
}
