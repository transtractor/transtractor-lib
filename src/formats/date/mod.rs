pub mod generate;

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
}