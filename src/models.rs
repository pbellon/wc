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

impl PartialEq for WcLineResult {
    fn eq(&self, other: &Self) -> bool {
        self.sizes == other.sizes && self.name == other.name
    }
}
