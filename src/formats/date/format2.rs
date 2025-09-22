use crate::formats::date::DateFormat;
use crate::formats::date::DateParts;

/// Format2: parses dates like "24 march 2020", "1 march 2020", "1 mar 2020"
pub struct Format2;

impl DateFormat for Format2 {
    fn num_items(&self) -> usize {
        3
    }

    /// Parses a date string and returns the UTC timestamp if valid.
    fn parse(&self, date_str: &str, _year_str: &str) -> Option<i64> {
        let re = regex::Regex::new(r"^\d{1,2} \w+ \d{4}$").unwrap();
        if !re.is_match(date_str) {
            return None;
        }
        let parts: Vec<&str> = date_str.split(' ').collect();
        if parts.len() != 3 {
            return None;
        }
        let date_parts = DateParts {
            day_str: parts[0].to_string(),
            month_str: parts[1].to_string(),
            year_str: parts[2].to_string(),
        };
        date_parts.to_utc_timestamp("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format2_parse() {
        let fmt = Format2;
        // 24 March 2020
        let ts = fmt.parse("24 march 2020", "");
        assert!(ts.is_some());
        // 1 Mar 2020
        let ts2 = fmt.parse("1 mar 2020", "");
        assert!(ts2.is_some());
        // Invalid
        assert_eq!(fmt.parse("mar 24 2020", ""), None);
        assert_eq!(fmt.parse("24 march", ""), None);
        assert_eq!(fmt.parse("24", ""), None);
        assert_eq!(fmt.parse("", ""), None);
    }
}