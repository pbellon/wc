use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read},
    path::Path,
    str::FromStr,
};

use clap::{command, Parser};

#[derive(Debug)]
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

#[derive(Parser)] // requires `derive` feature
#[command(version, about, long_about = None)]
pub struct CliParser {
    /// Outputs number of words if set
    #[arg(short = 'w', long = "words")]
    words: bool,

    /// Outputs number of bytes if set
    #[arg(short = 'b', long = "bytes")]
    bytes: bool,

    /// Outputs number of lines if set
    #[arg(short = 'l', long = "lines")]
    lines: bool,

    /// List of files to parse
    #[arg()]
    files: Vec<String>,
}

pub struct WcResult {
    sizes: Vec<usize>,
    name: String,
}

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

/// Get the list of calculus to perform on the list of files
/// depending on the various flags set (--lines, --bytes, --words)
/// will return all modes if no flags are detected
fn get_calculus(args: &CliParser) -> Vec<Mode> {
    let mut flags = Vec::new();
    if args.lines == true {
        flags.push(Mode::Lines);
    }
    if args.words == true {
        flags.push(Mode::Words);
    }
    if args.bytes == true {
        flags.push(Mode::Bytes);
    }

    if flags.is_empty() {
        flags.push(Mode::Lines);
        flags.push(Mode::Words);
        flags.push(Mode::Bytes);
    }

    flags
}

fn compute_for_file(path: &str, flags: &Vec<Mode>) -> Result<Vec<usize>, io::Error> {
    flags
        .iter()
        .map(|f| match f {
            Mode::Bytes => bytes_count(&path),
            Mode::Lines => lines_count(&path),
            Mode::Words => words_count(&path),
        })
        .collect()
}

pub fn print_results(results: &Vec<WcResult>) {
    if results.is_empty() {
        return;
    }

    // Calculate max width for the name column
    let name_width = results.iter().map(|r| r.name.len()).max().unwrap_or(0);

    // Assuming each Result has the same number of sizes, calculate max width for each size column
    let num_sizes = results[0].sizes.len();
    let mut sizes_width = vec![0; num_sizes];
    for result in results {
        for (i, size) in result.sizes.iter().enumerate() {
            sizes_width[i] = sizes_width[i].max(size.to_string().len());
        }
    }

    // Print each row
    for result in results {
        for (i, size) in result.sizes.iter().enumerate() {
            print!("{:width$} ", size, width = sizes_width[i]);
        }
        print!("{:width$}", result.name, width = name_width);
        println!();
    }
}

fn compute_total(results: &[WcResult]) -> WcResult {
    let mut sizes: Vec<usize> = vec![0; results[0].sizes.len()];
    sizes.fill(0);

    for result in results {
        for (i, &size) in result.sizes.iter().enumerate() {
            sizes[i] += size;
        }
    }

    WcResult {
        sizes,
        name: "Total".to_string(),
    }
}

pub fn run(args: &CliParser) -> io::Result<Vec<WcResult>> {
    let flags = get_calculus(&args);

    let mut results: Vec<WcResult> = Vec::new();

    for name in &args.files {
        let sizes = compute_for_file(&name, &flags)?;
        results.push(WcResult {
            sizes,
            name: name.to_string(),
        });
    }

    if results.len() > 1 {
        let total = compute_total(&results);
        results.push(total);
    }

    Ok(results)
}
