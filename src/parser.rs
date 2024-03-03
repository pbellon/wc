use clap::Parser;

use crate::models::Metric;

#[derive(Parser, Debug)] // requires `derive` feature
#[command(version, about, long_about = None)]
pub struct CliParser {
    /// Outputs number of words if set
    #[arg(short = 'w', long = "words")]
    pub words: bool,

    /// Outputs number of bytes if set
    #[arg(short = 'b', long = "bytes")]
    pub bytes: bool,

    /// Outputs number of lines if set
    #[arg(short = 'l', long = "lines")]
    pub lines: bool,

    /// List of files to parse
    #[arg()]
    pub files: Vec<String>,
}

impl CliParser {
    pub fn get_metrics(&self) -> Vec<Metric> {
        let mut metrics = Vec::new();

        if self.lines {
            metrics.push(Metric::Lines);
        }
        if self.words {
            metrics.push(Metric::Words);
        }
        if self.bytes {
            metrics.push(Metric::Bytes);
        }

        // If no flags are provided, assume all metrics are wanted
        if metrics.is_empty() {
            metrics = vec![Metric::Lines, Metric::Words, Metric::Bytes];
        }

        metrics
    }
}
