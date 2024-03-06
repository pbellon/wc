use std::ops::Add;

/// All possible metrics for file
#[derive(Debug)]
pub enum Metric {
    Lines,
    Words,
    Bytes,
}

impl PartialEq for Metric {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Counts(pub u64, pub u64, pub u64);

impl std::fmt::Display for Counts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} lines, {} words, {} bytes", self.0, self.1, self.2)
    }
}

impl Add for Counts {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Counts(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Counts {
    fn filter_by_metrics(&self, metrics: &[Metric]) -> Vec<u64> {
        metrics
            .iter()
            .map(|metric| match metric {
                Metric::Lines => self.0,
                Metric::Words => self.1,
                Metric::Bytes => self.2,
            })
            .collect()
    }
}

/// Represents a line in the program results, one per file + one for total
#[derive(Debug)]
pub struct WcLineResult {
    pub sizes: Vec<u64>,
    pub name: String,
}

impl WcLineResult {
    pub fn from(counts: &Counts, name: &str, metrics: &[Metric]) -> WcLineResult {
        let sizes = counts.filter_by_metrics(metrics);
        WcLineResult {
            sizes,
            name: String::from(name),
        }
    }
}

impl PartialEq for WcLineResult {
    fn eq(&self, other: &Self) -> bool {
        self.sizes == other.sizes && self.name == other.name
    }
}
