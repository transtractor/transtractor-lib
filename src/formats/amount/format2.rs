use crate::formats::amount::AmountFormat;
use regex::Regex;

/// Format2: parses amounts like "-$1,234.56", "$1,234.56", "$1,234.56-"
pub struct Format2;

impl AmountFormat for Format2 {
    fn num_terms(&self) -> usize { 1 }

    fn parse(&self, currency_str: &str) -> Option<f64> {
        let re = Regex::new(r"^-?\$\d{1,3}(,\d{3})*\.\d{2}(-|\s)?$").unwrap();
        if !re.is_match(currency_str) {
            return None;
        }
        // Remove commas
        let mut cleaned = currency_str.replace(',', "");
        // Determine sign
        let mut sign = 1.0;
        if cleaned.contains('-') {
            sign = -1.0;
            cleaned = cleaned.replace('-', "");
        }
        // Remove dollar sign
        cleaned = cleaned.replace('$', "");
        // Parse float
        match cleaned.parse::<f64>() {
            Ok(val) => Some(sign * val),
            Err(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format2() {
        let fmt = Format2;
        assert_eq!(fmt.parse("$1,234.56"), Some(1234.56));
        assert_eq!(fmt.parse("-$1,234.56"), Some(-1234.56));
        assert_eq!(fmt.parse("$1,234.56-"), Some(-1234.56));
        assert_eq!(fmt.parse("bad input"), None);
        assert_eq!(fmt.parse("1234.56"), None);
        assert_eq!(fmt.parse("$1234.56"), None);
        assert_eq!(fmt.parse("$1,234.5"), None);
        assert_eq!(fmt.parse("$1,234.567"), None);
        assert_eq!(fmt.parse("$1,000,234.56"), Some(100_0234.56));
    }
}