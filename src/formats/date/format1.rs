use crate::formats::date::DateFormat;
use crate::formats::date::DateParts;

/// Format1: parses dates like "24 mar", "1 mar", "01 mar"
pub struct Format1;

impl DateFormat for Format1 {
    fn num_items(&self) -> usize {
        2
    }

    /// Parses a date string and returns the UTC timestamp if valid.
    /// Requires a year_str argument.
    fn parse(&self, date_str: &str, year_str: &str) -> Option<i64> {
        let re = regex::Regex::new(r"^\d{1,2} \w+$").unwrap();
        if !re.is_match(date_str) {
            return None;
        }
        let parts: Vec<&str> = date_str.split(' ').collect();
        if parts.len() != 2 {
            return None;
        }
        let date_parts = DateParts {
            day_str: parts[0].to_string(),
            month_str: parts[1].to_string(),
            year_str: String::new(), // will use year_str argument in to_utc_timestamp
        };
        date_parts.to_utc_timestamp(year_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format1_parse() {
        let fmt = Format1;
        // 24 Mar 2023
        let ts = fmt.parse("24 mar", "2023");
        assert!(ts.is_some());
        // 01 Mar 2023
        let ts2 = fmt.parse("01 mar", "2023");
        assert!(ts2.is_some());
        // 9 Mar 2023
        let ts3 = fmt.parse("9 mar", "2023");
        assert!(ts3.is_some());
        // 30 Feb 2023 (invalid date)
        let ts4 = fmt.parse("30 feb", "2023");
        assert!(ts4.is_none());
        // Invalid
        assert_eq!(fmt.parse("mar 24", "2023"), None);
        assert_eq!(fmt.parse("", "2023"), None);
        assert_eq!(fmt.parse("24", "2023"), None);
    }
}