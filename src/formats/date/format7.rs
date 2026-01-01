use crate::formats::date::DateFormat;
use crate::formats::date::DateParts;

/// Format7: parses DD-MM-YYYY or DD-MM-YY dates like "24-03-2023", 24-3-2023", "24-03-23", "24-3-23"
pub struct Format7;

impl DateFormat for Format7 {
    fn num_items(&self) -> usize {
        1
    }

    /// Parses a date string and returns the UTC timestamp if valid.
    fn parse(&self, date_str: &str, _year_str: &str) -> Option<i64> {
        let re = regex::Regex::new(r"^\d{1,2}-\d{1,2}-\d{2,4}$").unwrap();
        if !re.is_match(date_str) {
            return None;
        }
        let parts: Vec<&str> = date_str.split('-').collect();
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
    fn test_format7_parse() {
        let fmt = Format7;
        // "24-03-2023" (DD-MM-YYYY)
        let ts = fmt.parse("24-03-2023", "");
        assert!(ts.is_some());
        // "24-3-2023" (DD-M-YYYY, single-digit month)
        let ts2 = fmt.parse("24-3-2023", "");
        assert!(ts2.is_some());
        // "24-03-23" (DD-MM-YY)
        let ts3 = fmt.parse("24-03-23", "");
        assert!(ts3.is_some());
        // "24-3-23" (DD-M-YY)
        let ts4 = fmt.parse("24-3-23", "");
        assert!(ts4.is_some());
        // "1-1-2023" (D-M-YYYY, both single-digit)
        let ts5 = fmt.parse("1-1-2023", "");
        assert!(ts5.is_some());
        // "31-12-2023" (December 31)
        let ts6 = fmt.parse("31-12-2023", "");
        assert!(ts6.is_some());
        // Invalid date (February 30)
        let ts7 = fmt.parse("30-02-2023", "");
        assert!(ts7.is_none());
        // Invalid format - wrong separator (slash)
        assert_eq!(fmt.parse("24/03/2023", ""), None);
        // Invalid format - wrong order (MM-DD-YYYY)
        // Note: This will parse but may give unexpected results
        // Invalid format - empty string
        assert_eq!(fmt.parse("", ""), None);
        // Invalid format - missing parts
        assert_eq!(fmt.parse("24-03", ""), None);
        // Invalid format - single digit year
        assert_eq!(fmt.parse("24-03-3", ""), None);
        // Invalid format - too many digits in day
        assert_eq!(fmt.parse("240-03-2023", ""), None);
    }
}
