use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read},
    path::Path,
};

fn get_reader(path: &str) -> io::Result<BufReader<File>> {
    let path = Path::new(path).canonicalize()?;
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    Ok(reader)
}

pub fn words_count(path: &str) -> io::Result<usize> {
    let reader = get_reader(path)?;
    let count = reader
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.split_ascii_whitespace().count())
        .sum();

    Ok(count)
}

pub fn lines_count(path: &str) -> io::Result<usize> {
    let reader = get_reader(path)?;
    let count = reader.lines().filter_map(Result::ok).count();
    Ok(count)
}

pub fn bytes_count(path: &str) -> io::Result<usize> {
    let mut reader = get_reader(path)?;
    let mut count = 0;
    let mut buffer = [0; 1024]; // Adjust buffer size as needed

    loop {
        match reader.read(&mut buffer) {
            Ok(0) => break, // End of file
            Ok(n) => count += n,
            Err(e) => return Err(e),
        }
    }

    Ok(count)
}
