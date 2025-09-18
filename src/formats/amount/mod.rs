pub mod format1;

/// Trait for amount formats.
pub trait AmountFormat {
    /// Number of space-delimited terms in the input string.
    const NUM_TERMS: usize;

    /// Parse the input string and return a float if valid.
    fn parse(&self, input: &str) -> Option<f64>;
}