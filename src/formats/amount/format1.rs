use crate::formats::amount::AmountFormat;
use regex::Regex;

/// Format1: parses amounts like "1,234.56", "-1,234.56", "1,234.56-"
pub struct Format1;

impl AmountFormat for Format1 {
    fn num_items(&self) -> usize { 1 }

    fn parse(&self, amount_str: &str) -> Option<f64> {
        let re = Regex::new(r"^-?\d{1,3}(,\d{3})*\.\d{2}(-|\s)?$").unwrap();
        if !re.is_match(amount_str) {
            return None;
        }
        // Remove commas
        let mut cleaned = amount_str.replace(',', "");
        // Determine sign
        let mut sign = 1.0;
        if cleaned.contains('-') {
            sign = -1.0;
            cleaned = cleaned.replace('-', "");
        }
        // Parse float
        match cleaned.parse::<f64>() {
            Ok(val) => Some(sign * val),
            Err(_) => None,
        }
    }
}

// Example usage:
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format1() {
        let fmt = Format1;
        assert_eq!(fmt.parse("1,234.56"), Some(1234.56));
        assert_eq!(fmt.parse("-1,234.56"), Some(-1234.56));
        assert_eq!(fmt.parse("1,234.56-"), Some(-1234.56));
        assert_eq!(fmt.parse("bad input"), None);
        assert_eq!(fmt.parse("$1234.56"), None);
        assert_eq!(fmt.parse("1234.5"), None);
        assert_eq!(fmt.parse("1234.567"), None);
        assert_eq!(fmt.parse("1234"), None);
        assert_eq!(fmt.parse("1234.56"), None);
        assert_eq!(fmt.parse("1,000,234.56"), Some(1000234.56));
    }
}