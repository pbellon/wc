use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

pub fn words_count(path: &PathBuf) -> io::Result<usize> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let count = reader
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.split_ascii_whitespace().count())
        .sum();

    Ok(count)
}

pub fn line_count(path: &PathBuf) -> io::Result<usize> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let count = reader.lines().filter_map(Result::ok).count();
    Ok(count)
}
