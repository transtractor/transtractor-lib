use crate::formats::date::DateFormat;
use crate::formats::date::DateParts;

/// Format11: parses dates like "Mar 24, 2023-Apr 24, 2023", "March 4, 2023-April 4, 2023"
/// Example: CapitalOne credit card statements
pub struct Format11;

impl DateFormat for Format11 {
    fn num_items(&self) -> usize {
        3
    }

    /// Parses a date string and returns the UTC timestamp if valid.
    /// Requires a year_str argument.
    fn parse(&self, date_str: &str, _year_str: &str) -> Option<i64> {
        // Only capture the "Mar 24, 2023-Apr" part
        let re = regex::Regex::new(r"^\w+ \d{1,2}, \d{4}-\w+").unwrap();
        if !re.is_match(date_str) {
            return None;
        }
        let parts: Vec<&str> = date_str
            .split(&[' ', ',', '-'][..])
            .filter(|s| !s.is_empty())
            .collect();
        if parts.len() < 4 {
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
    fn test_format11_parse() {
        let fmt = Format11;
        // "Mar 24, 2023-Apr" (abbreviated months)
        let ts = fmt.parse("Mar 24, 2023-Apr", "");
        assert!(ts.is_some());
        // "March 4, 2023-April" (full month names, single-digit day)
        let ts2 = fmt.parse("March 4, 2023-April", "");
        assert!(ts2.is_some());
        // "Jan 1, 2023-Feb" (January 1)
        let ts3 = fmt.parse("Jan 1, 2023-Feb", "");
        assert!(ts3.is_some());
        // "Dec 31, 2023-Jan" (December 31)
        let ts4 = fmt.parse("Dec 31, 2023-Jan", "");
        assert!(ts4.is_some());
        // "January 15, 2023-February" (full month name with two-digit day)
        let ts5 = fmt.parse("January 15, 2023-February", "");
        assert!(ts5.is_some());
        // Invalid date (February 30)
        let ts6 = fmt.parse("Feb 30, 2023-Mar", "");
        assert!(ts6.is_none());
        // Invalid format - missing comma
        assert_eq!(fmt.parse("Mar 24 2023-Apr", ""), None);
        // Invalid format - missing dash and second month
        assert_eq!(fmt.parse("Mar 24, 2023", ""), None);
        // Invalid format - empty string
        assert_eq!(fmt.parse("", ""), None);
        // Invalid format - missing day
        assert_eq!(fmt.parse("Mar 2023-Apr", ""), None);
        // Invalid format - wrong separator
        assert_eq!(fmt.parse("Mar 24-2023-Apr", ""), None);
        // Invalid format - two-digit year
        assert_eq!(fmt.parse("Mar 24, 23-Apr", ""), None);
    }
}
