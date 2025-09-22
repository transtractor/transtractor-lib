use crate::formats::date::DateFormat;
use crate::formats::date::DateParts;

/// Format3: parses dates like "march 24, 2020", "mar 1, 2020"
pub struct Format3;

impl DateFormat for Format3 {
    fn num_items(&self) -> usize {
        3
    }

    /// Parses a date string and returns the UTC timestamp if valid.
    fn parse(&self, date_str: &str, _year_str: &str) -> Option<i64> {
        let re = regex::Regex::new(r"^\w+ \d{1,2}, \d{4}$").unwrap();
        if !re.is_match(date_str) {
            return None;
        }
        // Remove comma and split
        let cleaned = date_str.replace(",", "");
        let parts: Vec<&str> = cleaned.split(' ').collect();
        if parts.len() != 3 {
            return None;
        }
        let date_parts = DateParts {
            day_str: parts[1].to_string(),
            month_str: parts[0].to_string(),
            year_str: parts[2].to_string(),
        };
        date_parts.to_utc_timestamp("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format3_parse() {
        let fmt = Format3;
        // "march 24, 2020"
        let ts = fmt.parse("march 24, 2020", "");
        assert!(ts.is_some());
        // "mar 1, 2020"
        let ts2 = fmt.parse("mar 1, 2020", "");
        assert!(ts2.is_some());
        // Invalid
        assert_eq!(fmt.parse("24 march 2020", ""), None);
        assert_eq!(fmt.parse("march 24 2020", ""), None);
        assert_eq!(fmt.parse("march", ""), None);
        assert_eq!(fmt.parse("", ""), None);
    }
}