mod models;
mod parser;

use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

pub use crate::models::{Metric, WcLineResult};

pub use crate::parser::{get_metrics_from_args, CliParser};

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

fn sizes_for_metrics(counts: (u64, u64, u64), metrics: &[Metric]) -> Vec<u64> {
    let mut sizes: Vec<u64> = Vec::with_capacity(metrics.len());
    for metric in metrics {
        match metric {
            Metric::Lines => sizes.push(counts.0),
            Metric::Words => sizes.push(counts.1),
            Metric::Bytes => sizes.push(counts.2),
        }
    }
    sizes
}

pub fn process_files(pathes: &Vec<String>, metrics: &[Metric]) -> io::Result<Vec<WcLineResult>> {
    let mut total_counts: (u64, u64, u64) = (0, 0, 0);
    let mut results: Vec<WcLineResult> = Vec::new();

    for path in pathes {
        let file_counts = count_file(path)?;
        let sizes = sizes_for_metrics(file_counts, metrics);
        // Happen current counts to global res
        total_counts.0 += file_counts.0;
        total_counts.1 += file_counts.1;
        total_counts.2 += file_counts.2;
        results.push(WcLineResult {
            sizes,
            name: path.to_string(),
        });
    }

    // only happen global total if command executed on more than 1 file
    if results.len() > 1 {
        let sizes = sizes_for_metrics(total_counts, metrics);
        results.push(WcLineResult {
            sizes,
            name: "total".to_string(),
        });
    }

    Ok(results)
}
