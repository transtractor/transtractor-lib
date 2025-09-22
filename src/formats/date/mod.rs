pub mod format1;
pub mod format2;
pub mod format3;
pub mod format4;
pub mod format5;
pub mod generate;

use crate::formats::date::{format1::Format1, format2::Format2, format3::Format3, format4::Format4, format5::Format5};
use crate::formats::date::generate::{parse_day, parse_month, parse_year};


/// Trait for date formats.
pub trait DateFormat {
    /// Number of space-delimited items in the input string.
    fn num_items(&self) -> usize;

    /// Parse the input string and return a UTC timestamp (milliseconds since epoch) if valid.
    fn parse(&self, input: &str, year_str: &str) -> Option<i64>;
}

/// Stores day, month, and year strings and can convert to a UTC timestamp.
#[derive(Debug, Clone)]
pub struct DateParts {
    pub day_str: String,
    pub month_str: String,
    pub year_str: String,
}

impl DateParts {
    pub fn new(day_str: String, month_str: String, year_str: String) -> Self {
        Self {
            day_str,
            month_str,
            year_str,
        }
    }

    /// Attempts to convert the stored strings to a UTC timestamp (milliseconds since epoch).
    /// If self.year_str is empty, uses the input arg year_str.
    /// If both are empty, panics.
    /// If self.year_str is not empty, uses it even if the input arg is not empty.
    pub fn to_utc_timestamp(&self, year_str: &str) -> Option<i64> {
        let day = parse_day(&self.day_str)? as u32;
        let month = parse_month(&self.month_str)? as u32;

        // Determine which year string to use
        let year_source = if !self.year_str.trim().is_empty() {
            &self.year_str
        } else if !year_str.trim().is_empty() {
            year_str
        } else {
            panic!("No year string provided to to_utc_timestamp");
        };

        let year = parse_year(year_source)? as i32;

        let date = chrono::NaiveDate::from_ymd_opt(year, month, day)?;
        let datetime = date.and_hms_opt(0, 0, 0)?;
        Some(datetime.and_utc().timestamp_millis())
    }
}

/// Dispatcher for multiple date formats.
pub struct MultiDateFormatParser {
    parsers: Vec<Box<dyn DateFormat>>,
}

impl MultiDateFormatParser {
    /// Create a new dispatcher from a list of format names.
    pub fn new(format_names: &[&str]) -> Self {
        // Collect (name, num_items) pairs
        let mut formats: Vec<(&str, usize)> = format_names.iter().map(|&name| {
            let num_items = match name {
                "format1" => Format1.num_items(),
                "format2" => Format2.num_items(),
                "format3" => Format3.num_items(),
                "format4" => Format4.num_items(),
                "format5" => Format5.num_items(),
                _ => 0,
            };
            (name, num_items)
        }).collect();

        // Sort by num_items descending
        formats.sort_by(|a, b| b.1.cmp(&a.1));

        // Instantiate parsers in sorted order
        let mut parsers: Vec<Box<dyn DateFormat>> = Vec::new();
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
        MultiDateFormatParser { parsers }
    }

    /// Try parsing with each format in order, returning the first successful result.
    pub fn parse(&self, input: &str, year_str: &str) -> Option<i64> {
        for parser in &self.parsers {
            if let Some(val) = parser.parse(input, year_str) {
                return Some(val);
            }
        }
        None
    }

    /// Returns the maximum number of items among all formats.
    pub fn max_items(&self) -> usize {
        self.parsers.iter().map(|p| p.num_items()).max().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_date() {
        let dp = DateParts {
            day_str: "15".to_string(),
            month_str: "Feb".to_string(),
            year_str: "2023".to_string(),
        };
        assert_eq!(dp.to_utc_timestamp(""), Some(1676419200000)); // 2023-02-15T00:00:00Z
    }

    #[test]
    fn test_invalid_date() {
        let dp = DateParts {
            day_str: "32".to_string(),
            month_str: "Feb".to_string(),
            year_str: "2023".to_string(),
        };
        assert_eq!(dp.to_utc_timestamp(""), None);
    }

    #[test]
    fn test_invalid_month() {
        let dp = DateParts {
            day_str: "15".to_string(),
            month_str: "Foo".to_string(),
            year_str: "2023".to_string(),
        };
        assert_eq!(dp.to_utc_timestamp(""), None);
    }

    #[test]
    fn test_invalid_year() {
        let dp = DateParts {
            day_str: "15".to_string(),
            month_str: "Feb".to_string(),
            year_str: "abcd".to_string(),
        };
        assert_eq!(dp.to_utc_timestamp(""), None);
    }

    #[test]
    fn test_to_utc_timestamp_uses_input_year_if_self_year_empty() {
        let dp = DateParts {
            day_str: "15".to_string(),
            month_str: "Feb".to_string(),
            year_str: "".to_string(),
        };
        assert_eq!(dp.to_utc_timestamp("2023"), Some(1676419200000)); // 2023-02-15T00:00:00Z
    }

    #[test]
    #[should_panic(expected = "No year string provided to to_utc_timestamp")]
    fn test_to_utc_timestamp_panics_if_no_year_provided() {
        let dp = DateParts {
            day_str: "15".to_string(),
            month_str: "Feb".to_string(),
            year_str: "".to_string(),
        };
        dp.to_utc_timestamp("");
    }

    #[test]
    fn test_multi_date_format_parser() {
        let multi_fmt = MultiDateFormatParser::new(&["format1", "format2", "format3", "format4", "format5"]);
        // Should parse using format1
        assert!(multi_fmt.parse("24 mar", "2023").is_some());
        // Should parse using format2
        assert!(multi_fmt.parse("24 march 2020", "").is_some());
        // Should parse using format3
        assert!(multi_fmt.parse("march 24, 2020", "").is_some());
        // Should parse using format4
        assert!(multi_fmt.parse("24/3/2020", "").is_some());
        // Should parse using format5
        assert!(multi_fmt.parse("24/3/20", "").is_some());
        // Should not parse invalid
        assert_eq!(multi_fmt.parse("foo", "2023"), None);
    }

    #[test]
    fn test_max_items() {
        let multi_fmt = MultiDateFormatParser::new(&["format1", "format3", "format5"]);
        assert_eq!(multi_fmt.max_items(), 3);

        let multi_fmt2 = MultiDateFormatParser::new(&["format1"]);
        assert_eq!(multi_fmt2.max_items(), 2);

        let multi_fmt3 = MultiDateFormatParser::new(&[]);
        assert_eq!(multi_fmt3.max_items(), 0);
    }
}