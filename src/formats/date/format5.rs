use crate::formats::date::DateFormat;
use crate::formats::date::DateParts;

/// Format5: parses dates like "24/3/20", "01/03/20", "24/03/20"
pub struct Format5;

impl DateFormat for Format5 {
    fn num_items(&self) -> usize {
        3
    }

    /// Parses a date string and returns the UTC timestamp if valid.
    fn parse(&self, date_str: &str, _year_str: &str) -> Option<i64> {
        let re = regex::Regex::new(r"^\d{1,2}/\d{1,2}/\d{2}$").unwrap();
        if !re.is_match(date_str) {
            return None;
        }
        let parts: Vec<&str> = date_str.split('/').collect();
        if parts.len() != 3 {
            return None;
        }
        // Convert 2-digit year to 4-digit year using parse_year
        let year = crate::formats::date::generate::parse_year(parts[2])?;
        let date_parts = DateParts {
            day_str: parts[0].to_string(),
            month_str: parts[1].to_string(),
            year_str: year.to_string(),
        };
        date_parts.to_utc_timestamp("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format5_parse() {
        let fmt = Format5;
        // "24/3/20"
        let ts = fmt.parse("24/3/20", "");
        assert!(ts.is_some());
        // "01/03/20"
        let ts2 = fmt.parse("01/03/20", "");
        assert!(ts2.is_some());
        // "24/03/20"
        let ts3 = fmt.parse("24/03/20", "");
        assert!(ts3.is_some());
        // Invalid
        assert_eq!(fmt.parse("24-03-20", ""), None);
        assert_eq!(fmt.parse("24/03", ""), None);
        assert_eq!(fmt.parse("", ""), None);
    }
}