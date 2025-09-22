pub mod generate;

use crate::formats::date::generate::{parse_day, parse_month, parse_year};

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
    /// Returns None if any part is invalid.
    pub fn to_utc_timestamp(&self) -> Option<i64> {
        let day = parse_day(&self.day_str)? as u32;
        let month = parse_month(&self.month_str)? as u32;
        let year = parse_year(&self.year_str)? as i32;

        let date = chrono::NaiveDate::from_ymd_opt(year, month + 1, day)?;
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
        assert_eq!(dp.to_utc_timestamp(), Some(1676419200000)); // 2023-02-15T00:00:00Z
    }

    #[test]
    fn test_invalid_date() {
        let dp = DateParts {
            day_str: "32".to_string(),
            month_str: "Feb".to_string(),
            year_str: "2023".to_string(),
        };
        assert_eq!(dp.to_utc_timestamp(), None);
    }

    #[test]
    fn test_invalid_month() {
        let dp = DateParts {
            day_str: "15".to_string(),
            month_str: "Foo".to_string(),
            year_str: "2023".to_string(),
        };
        assert_eq!(dp.to_utc_timestamp(), None);
    }

    #[test]
    fn test_invalid_year() {
        let dp = DateParts {
            day_str: "15".to_string(),
            month_str: "Feb".to_string(),
            year_str: "abcd".to_string(),
        };
        assert_eq!(dp.to_utc_timestamp(), None);
    }
}