use crate::formats::date::DateFormat;
use crate::formats::date::DateParts;

/// Format9: parses MM/DD/YYYY or MM/DD/YY dates like "03/24/2023", "3/24/2023", "03/24/23", "3/24/23"
pub struct Format9;

impl DateFormat for Format9 {
    fn num_items(&self) -> usize {
        1
    }

    /// Parses a date string and returns the UTC timestamp if valid.
    fn parse(&self, date_str: &str, _year_str: &str) -> Option<i64> {
        let re = regex::Regex::new(r"^\d{1,2}/\d{1,2}/\d{2,4}$").unwrap();
        if !re.is_match(date_str) {
            return None;
        }
        let parts: Vec<&str> = date_str.split('/').collect();
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
    fn test_format9_parse() {
        let fmt = Format9;
        // "03/24/2023" (MM/DD/YYYY)
        let ts = fmt.parse("03/24/2023", "");
        assert!(ts.is_some());
        // "3/24/2023" (M/DD/YYYY, single-digit month)
        let ts2 = fmt.parse("3/24/2023", "");
        assert!(ts2.is_some());
        // "03/24/23" (MM/DD/YY)
        let ts3 = fmt.parse("03/24/23", "");
        assert!(ts3.is_some());
        // "3/24/23" (M/DD/YY)
        let ts4 = fmt.parse("3/24/23", "");
        assert!(ts4.is_some());
        // "1/1/2023" (M/D/YYYY, both single-digit)
        let ts5 = fmt.parse("1/1/2023", "");
        assert!(ts5.is_some());
        // "12/31/2023" (December 31)
        let ts6 = fmt.parse("12/31/2023", "");
        assert!(ts6.is_some());
        // Invalid date (February 30)
        let ts7 = fmt.parse("02/30/2023", "");
        assert!(ts7.is_none());
        // Invalid format - wrong separator (dash)
        assert_eq!(fmt.parse("03-24-2023", ""), None);
        // Invalid format - empty string
        assert_eq!(fmt.parse("", ""), None);
        // Invalid format - missing parts
        assert_eq!(fmt.parse("03/24", ""), None);
        // Invalid format - single digit year
        assert_eq!(fmt.parse("03/24/3", ""), None);
        // Invalid format - too many digits in month
        assert_eq!(fmt.parse("003/24/2023", ""), None);
    }
}