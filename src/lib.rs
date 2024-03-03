use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

use clap::{command, Parser};

#[derive(Debug)]
pub enum Mode {
    Words,
    Lines,
    Bytes,
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
    sizes: Vec<u64>,
    name: String,
}

pub fn count_file(path: &str) -> io::Result<(u64, u64, u64)> {
    let path = Path::new(path).canonicalize()?;
    let file = File::open(path)?;

    let bytes = file.metadata()?.len();
    let reader = BufReader::new(file);

    let mut lines = 0;
    let mut words = 0;

    for line in reader.lines().filter_map(Result::ok) {
        lines += 1;
        words += line.split_ascii_whitespace().count();
    }

    Ok((lines as u64, words as u64, bytes))
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

fn sizes_for_flags(counts: (u64, u64, u64), flags: &[Mode]) -> Vec<u64> {
    let mut sizes = Vec::with_capacity(flags.len());
    for flag in flags {
        match flag {
            Mode::Lines => sizes.push(counts.0),
            Mode::Words => sizes.push(counts.1),
            Mode::Bytes => sizes.push(counts.2),
        }
    }
    sizes
}

pub fn run(args: &CliParser) -> io::Result<Vec<WcResult>> {
    let flags = get_calculus(&args);

    let mut total: (u64, u64, u64) = (0, 0, 0);

    let mut results: Vec<WcResult> = Vec::new();

    for name in &args.files {
        let counts = count_file(name)?;
        let sizes = sizes_for_flags(counts, &flags);
        // Happen current counts to global res
        total.0 += counts.0;
        total.1 += counts.1;
        total.1 += counts.2;
        results.push(WcResult {
            sizes,
            name: name.to_string(),
        });
    }

    // only happen global total if command executed on more than 1 file
    if results.len() > 1 {
        let sizes = sizes_for_flags(total, &flags);
        results.push(WcResult {
            sizes,
            name: "total".to_string(),
        });
    }

    Ok(results)
}
