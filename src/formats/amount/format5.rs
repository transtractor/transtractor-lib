use crate::formats::amount::AmountFormat;

/// Format5: parses "Nil" or "nil" as 0, anything else as None
pub struct Format5;

impl AmountFormat for Format5 {
    fn num_items(&self) -> usize { 1 }

    fn parse(&self, currency_str: &str) -> Option<f64> {
        if currency_str.trim().eq_ignore_ascii_case("nil") {
            Some(0.0)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format5() {
        let fmt = Format5;
        assert_eq!(fmt.parse("Nil"), Some(0.0));
        assert_eq!(fmt.parse("nil"), Some(0.0));
        assert_eq!(fmt.parse(" NIL "), Some(0.0));
        assert_eq!(fmt.parse("none"), None);
        assert_eq!(fmt.parse("0"), None);
    }
}