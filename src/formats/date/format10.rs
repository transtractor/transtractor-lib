use crate::formats::date::DateFormat;
use crate::formats::date::DateParts;

/// Format10: parses MMM DD dates like "Mar 24", "Mar 4", "March 4"
pub struct Format10;

impl DateFormat for Format10 {
    fn num_items(&self) -> usize {
        2
    }

    /// Parses a date string and returns the UTC timestamp if valid.
    fn parse(&self, date_str: &str, year_str: &str) -> Option<i64> {
        let re = regex::Regex::new(r"^\w+ \d{1,2}$").unwrap();
        if !re.is_match(date_str) {
            return None;
        }
        let parts: Vec<&str> = date_str.split(' ').collect();
        if parts.len() != 2 {
            return None;
        }
        let date_parts = DateParts {
            day_str: parts[1].to_string(),
            month_str: parts[0].to_string(),
            year_str: String::new(), // will use year_str argument in to_utc_timestamp
        };
        date_parts.to_utc_timestamp(year_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format10_parse() {
        let fmt = Format10;
        // "Mar 24" (abbreviated month)
        let ts = fmt.parse("Mar 24", "2023");
        assert!(ts.is_some());
        // "Mar 4" (single-digit day)
        let ts2 = fmt.parse("Mar 4", "2023");
        assert!(ts2.is_some());
        // "March 4" (full month name)
        let ts3 = fmt.parse("March 4", "2023");
        assert!(ts3.is_some());
        // "Jan 1" (January 1)
        let ts4 = fmt.parse("Jan 1", "2023");
        assert!(ts4.is_some());
        // "Dec 31" (December 31)
        let ts5 = fmt.parse("Dec 31", "2023");
        assert!(ts5.is_some());
        // "January 15"
        let ts6 = fmt.parse("January 15", "2023");
        assert!(ts6.is_some());
        // Invalid date (February 30)
        let ts7 = fmt.parse("Feb 30", "2023");
        assert!(ts7.is_none());
        // Invalid format - wrong order (DD MMM)
        // Note: "24 Mar" will match regex but fail parsing
        let ts8 = fmt.parse("24 Mar", "2023");
        assert!(ts8.is_none());
        // Invalid format - empty string
        assert_eq!(fmt.parse("", "2023"), None);
        // Invalid format - missing parts
        assert_eq!(fmt.parse("Mar", "2023"), None);
        // Invalid format - includes year
        assert_eq!(fmt.parse("Mar 24 2023", "2023"), None);
    }
}
