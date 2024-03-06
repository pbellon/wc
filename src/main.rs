// use std::env::args;
use atty::Stream;
use clap::{CommandFactory, Parser};
use std::io::{self};
use wc::{process_files, CliParser, WcLineResult};

pub fn print_results(results: &[WcLineResult]) {
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

fn main() -> Result<(), io::Error> {
    let args = CliParser::parse();
    let metrics = args.get_metrics();
    let files = args.files.unwrap_or(vec![]);

    if atty::is(Stream::Stdin) && files.is_empty() {
        CliParser::command().print_help()?;
        println!();
        return Ok(());
    }

    let results = process_files(&files, &metrics)?;
    print_results(&results);

    Ok(())
}
