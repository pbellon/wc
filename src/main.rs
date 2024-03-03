// use std::env::args;
use std::io;
use std::str::FromStr;

use clap::Parser;
use wc::{bytes_count, lines_count, words_count};
// use wc::{bytes_count, lines_count, words_count};

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

struct WcResult {
    sizes: Vec<usize>,
    name: String,
}

// fn old_main() -> Result<(), Box<dyn std::error::Error>> {
//     let args: Vec<String> = args().collect();

//     if args.len() < 3 {
//         eprintln!("Usage: {} <file_path> (-w | -l)", args[0]);
//         std::process::exit(1);
//     }

//     let path = &args[1];
//     let mode = Mode::from_str(&args[2])?;

//     let result = match mode {
//         Mode::Words => words_count(&path)?,
//         Mode::Lines => lines_count(&path)?,
//         Mode::Bytes => bytes_count(&path)?,
//     };

//     println!("{result} {}", path);
//     Ok(())
// }

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

fn print_results(results: &Vec<WcResult>) {
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

fn main() -> Result<(), io::Error> {
    let args = CliParser::parse();

    let flags = get_calculus(&args);

    let mut results: Vec<WcResult> = Vec::new();

    for name in args.files {
        let sizes = compute_for_file(&name, &flags)?;
        results.push(WcResult { sizes, name });
    }

    if results.len() > 1 {
        let total = compute_total(&results);
        results.push(total);
    }

    print_results(&results);

    Ok(())
}
