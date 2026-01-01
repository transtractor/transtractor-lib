use crate::formats::date::DateFormat;
use crate::formats::date::DateParts;

/// Format6: parses MM/DD or M/D dates like "03/12", "3/12", "3/2"
pub struct Format6;

impl DateFormat for Format6 {
    fn num_items(&self) -> usize {
        1
    }

    /// Parses a date string and returns the UTC timestamp if valid.
    fn parse(&self, date_str: &str, year_str: &str) -> Option<i64> {
        let re = regex::Regex::new(r"^\d{1,2}/\d{1,2}$").unwrap();
        if !re.is_match(date_str) {
            return None;
        }
        let parts: Vec<&str> = date_str.split('/').collect();
        if parts.len() != 2 {
            return None;
        }
        let date_parts = DateParts {
            day_str: parts[1].to_string(),
            month_str: parts[0].to_string(),
            year_str: year_str.to_string(),
        };
        date_parts.to_utc_timestamp("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format6_parse() {
        let fmt = Format6;
        // "03/12" (March 12)
        let ts = fmt.parse("03/12", "2023");
        assert!(ts.is_some());
        // "3/12" (March 12, single-digit month)
        let ts2 = fmt.parse("3/12", "2023");
        assert!(ts2.is_some());
        // "3/2" (March 2, both single-digit)
        let ts3 = fmt.parse("3/2", "2023");
        assert!(ts3.is_some());
        // "12/25" (December 25)
        let ts4 = fmt.parse("12/25", "2023");
        assert!(ts4.is_some());
        // "01/01" (January 1)
        let ts5 = fmt.parse("01/01", "2023");
        assert!(ts5.is_some());
        // Invalid date (February 30)
        let ts6 = fmt.parse("02/30", "2023");
        assert!(ts6.is_none());
        // Invalid format - wrong separator
        assert_eq!(fmt.parse("03-12", "2023"), None);
        // Invalid format - includes year
        assert_eq!(fmt.parse("03/12/23", "2023"), None);
        // Invalid format - too many digits
        assert_eq!(fmt.parse("003/12", "2023"), None);
        // Invalid format - empty string
        assert_eq!(fmt.parse("", "2023"), None);
        // Invalid format - month only
        assert_eq!(fmt.parse("03", "2023"), None);
        // Invalid format - month out of range
        assert_eq!(fmt.parse("13/01", "2023"), None);
    }
}
