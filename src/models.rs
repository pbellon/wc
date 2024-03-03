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

/// Represents a line in the program results, one per file + one for total
#[derive(Debug)]
pub struct WcLineResult {
    pub sizes: Vec<u64>,
    pub name: String,
}

impl WcLineResult {
    pub fn from(counts: &(u64, u64, u64), name: &str, metrics: &[Metric]) -> WcLineResult {
        let sizes = metrics
            .iter()
            .map(|metric| match metric {
                Metric::Lines => counts.0,
                Metric::Words => counts.1,
                Metric::Bytes => counts.2,
            })
            .collect();

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
