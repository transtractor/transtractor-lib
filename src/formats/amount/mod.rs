pub mod format1;
pub mod format2;
pub mod format3;
pub mod format4;
pub mod format5;

use format1::Format1;
use format2::Format2;
use format3::Format3;
use format4::Format4;
use format5::Format5;


/// Trait for amount formats.
pub trait AmountFormat {
    /// Number of space-delimited terms in the input string.
    fn num_terms(&self) -> usize;

    /// Parse the input string and return a float if valid.
    fn parse(&self, input: &str) -> Option<f64>;
}

/// Dispatcher for multiple amount formats.
pub struct MultiAmountFormatParser {
    parsers: Vec<Box<dyn AmountFormat>>,
}

impl MultiAmountFormatParser {
    /// Create a new dispatcher from a list of format names.
    pub fn new(format_names: &[&str]) -> Self {
        // Collect (name, NUM_TERMS) pairs
        let mut formats: Vec<(&str, usize)> = format_names.iter().map(|&name| {
            let num_terms = match name {
                "format1" => Format1.num_terms(),
                "format2" => Format2.num_terms(),
                "format3" => Format3.num_terms(),
                "format4" => Format4.num_terms(),
                "format5" => Format5.num_terms(),
                _ => 0,
            };
            (name, num_terms)
        }).collect();

        // Sort by NUM_TERMS descending
        formats.sort_by(|a, b| b.1.cmp(&a.1));

        // Instantiate parsers in sorted order
        let mut parsers: Vec<Box<dyn AmountFormat>> = Vec::new();
        for &(name, _) in &formats {
            match name {
                "format1" => parsers.push(Box::new(Format1)),
                "format2" => parsers.push(Box::new(Format2)),
                "format3" => parsers.push(Box::new(Format3)),
                "format4" => parsers.push(Box::new(Format4)),
                "format5" => parsers.push(Box::new(Format5)),
                _ => {}
            }
        }
        MultiAmountFormatParser { parsers }
    }

    /// Try parsing with each format in order, returning the first successful result.
    pub fn parse(&self, input: &str) -> Option<f64> {
        for parser in &self.parsers {
            if let Some(val) = parser.parse(input) {
                return Some(val);
            }
        }
        None
    }

    /// Get the maximum number of terms among the included formats.
    pub fn max_terms(&self) -> usize {
        self.parsers.iter().map(|p| p.num_terms()).max().unwrap_or(0)
    }
}

// Example usage:
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_multi_amount_format_parser() {
        let multi_fmt1 = MultiAmountFormatParser::new(&["format1", "format2"]);
        assert_eq!(multi_fmt1.parse("1,234.56"), Some(1234.56));
        assert_eq!(multi_fmt1.parse("-$1,234.56"), Some(-1234.56)); // format2
        assert_eq!(multi_fmt1.parse("$1,234.56 DR"), None); // format3 not included
    }

    #[test]
    fn test_max_terms() {
        let multi_fmt = MultiAmountFormatParser::new(&["format1", "format3", "format5"]);
        // format1: 1 term, format3: 2 terms, format5: 1 term
        assert_eq!(multi_fmt.max_terms(), 2);

        let multi_fmt2 = MultiAmountFormatParser::new(&["format1", "format5"]);
        assert_eq!(multi_fmt2.max_terms(), 1);

        let multi_fmt3 = MultiAmountFormatParser::new(&[]);
        assert_eq!(multi_fmt3.max_terms(), 0);
    }
}