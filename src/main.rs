use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn words_count(path: &str) -> usize {
    let file = File::open(&path).expect("Cannot open file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap().split_whitespace().count())
        .reduce(|acc, e| acc + e)
        .unwrap()
}

fn line_count(path: &str) -> usize {
    let file = File::open(&path).expect("Cannot open file");
    let reader = BufReader::new(file);
    reader.lines().count()
}

fn count(path: &str, mode: &str) -> usize {
    if mode == "-w" {
        words_count(&path)
    } else if mode == "-l" {
        line_count(&path)
    } else {
        panic!("Unknown mode")
    }
}

fn main() {
    // Args parsing: wc <path> (-w|-l)
    // first arg: path of file
    let path = env::args().nth(1).expect("No path given");
    // -w | -l
    let mode = env::args().nth(2).unwrap_or("-w".to_string());

    let result = count(&path, &mode);
    println!("{result} {}", &path);
}
