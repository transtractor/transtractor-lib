use crate::formats::date::DateFormat;
use crate::formats::date::DateParts;

/// Format13: parses YYYY-MM-DD dates like "2023-03-24", "2023-3-24"
pub struct Format13;

impl DateFormat for Format13 {
    fn num_items(&self) -> usize {
        1
    }

    /// Parses a date string and returns the UTC timestamp if valid.
    fn parse(&self, date_str: &str, _year_str: &str) -> Option<i64> {
        let re = regex::Regex::new(r"^\d{4}-\d{1,2}-\d{1,2}$").unwrap();
        if !re.is_match(date_str) {
            return None;
        }
        let parts: Vec<&str> = date_str.split('-').collect();
        if parts.len() != 3 {
            return None;
        }
        let date_parts = DateParts {
            day_str: parts[2].to_string(),
            month_str: parts[1].to_string(),
            year_str: parts[0].to_string(),
        };
        date_parts.to_utc_timestamp("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format13_parse() {
        let fmt = Format13;
        // "2023-03-24" (YYYY-MM-DD)
        let ts = fmt.parse("2023-03-24", "");
        assert!(ts.is_some());
        // "2023-3-24" (YYYY-M-DD, single-digit month)
        let ts2 = fmt.parse("2023-3-24", "");
        assert!(ts2.is_some());
        // "2023-03-4" (YYYY-MM-D, single-digit day)
        let ts3 = fmt.parse("2023-03-4", "");
        assert!(ts3.is_some());
        // "2023-3-4" (YYYY-M-D, both single-digit)
        let ts4 = fmt.parse("2023-3-4", "");
        assert!(ts4.is_some());
        // "2023-01-01" (January 1)
        let ts5 = fmt.parse("2023-01-01", "");
        assert!(ts5.is_some());
        // "2023-12-31" (December 31)
        let ts6 = fmt.parse("2023-12-31", "");
        assert!(ts6.is_some());
        // Invalid date (February 30)
        let ts7 = fmt.parse("2023-02-30", "");
        assert!(ts7.is_none());
        // Invalid format - wrong separator (slash)
        assert_eq!(fmt.parse("2023/03/24", ""), None);
        // Invalid format - empty string
        assert_eq!(fmt.parse("", ""), None);
        // Invalid format - missing parts
        assert_eq!(fmt.parse("2023-03", ""), None);
        // Invalid format - two-digit year
        assert_eq!(fmt.parse("23-03-24", ""), None);
        // Invalid format - three-digit year
        assert_eq!(fmt.parse("202-03-24", ""), None);
        // Invalid format - too many digits in month
        assert_eq!(fmt.parse("2023-003-24", ""), None);
    }
}
