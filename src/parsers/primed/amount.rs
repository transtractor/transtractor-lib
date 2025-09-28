use crate::structs::TextItem;
use crate::parsers::base::AmountParser;
use crate::parsers::base::ParserPrimer;

pub struct PrimedAmountParser {
    primer_parser: ParserPrimer,
    amount_parser: AmountParser,
    same_x1: bool,
    x1_tol: i32,
    same_y1: bool,
    y1_tol: i32,
    invert: bool,
}

impl PrimedAmountParser {
    pub fn new(
        primer_terms: &[&str],
        amount_formats: &[&str],
        same_x1: bool,
        x1_tol: i32,
        same_y1: bool,
        y1_tol: i32,
        invert: bool,
    ) -> Self {
        Self {
            primer_parser: ParserPrimer::new(primer_terms),
            amount_parser: AmountParser::new(amount_formats),
            same_x1,
            x1_tol,
            same_y1,
            y1_tol,
            invert,
        }
    }

    pub fn parse_items(&mut self, items: &[TextItem]) -> usize {
        // No items to parse
        if items.is_empty() {
            return 0;
        }

        // Return if value already set
        if self.amount_parser.value.is_some() {
            return 0;
        }

        // Primer not primed, or re-prime if term found again
        let consumed_primer = self.primer_parser.parse_items(items);
        if consumed_primer > 0 {
            return consumed_primer;
        }

        // Must be primed to look for amount
        if !self.primer_parser.primed {
            return 0; // Primer not found yet
        }

        // Primer is primed, look for amount
        let consumed = self.amount_parser.parse_items(items);
        if consumed == 0 {
            return 0; // No amount found
        }

        // Both primer and amount found, check conditions
        let amount_item = self.amount_parser.text_item();
        let primer_item = self.primer_parser.text_item();

        // Check x1, y1 and page conditions
        let x1_ok = if self.same_x1 {
            (amount_item.x1 - primer_item.x1).abs() <= self.x1_tol
        } else {
            true
        };
        let y1_ok = if self.same_y1 {
            (amount_item.y1 - primer_item.y1).abs() <= self.y1_tol
        } else {
            true
        };
        let page_ok = amount_item.page == primer_item.page;

        // Return 0 if any condition fails
        if !x1_ok || !y1_ok || !page_ok {
            // Reset amount parser state
            self.amount_parser.reset();
            return 0;
        }

        // All conditions met, finalize
        if self.invert {
            self.amount_parser.invert();
        }
        consumed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::TextItem;

    fn make_text_item(text: &str, x1: i32, y1: i32, page: usize) -> TextItem {
        TextItem {
            text: text.to_string(),
            x1,
            y1,
            x2: x1 + 10,
            y2: y1 + 10,
            page,
        }
    }

    #[test]
    fn test_primer_and_amount_success() {
        let mut parser = PrimedAmountParser::new(&["PRIME"], &["format1"], true, 5, true, 5, false);
        let items = vec![
            make_text_item("PRIME", 100, 200, 1),
            make_text_item("1,234.56", 102, 202, 1),
        ];
        // First call primes the parser
        let consumed_primer = parser.parse_items(&items);
        assert_eq!(consumed_primer, 1);
        assert!(parser.primer_parser.primed);
        assert!(parser.amount_parser.value.is_none());

        // Second call parses the amount
        let consumed_amount = parser.parse_items(&items[1..]);
        assert_eq!(consumed_amount, 1);
        assert_eq!(parser.amount_parser.value, Some(1234.56));
    }

    #[test]
    fn test_primer_and_amount_invert() {
        let mut parser = PrimedAmountParser::new(&["PRIME"], &["format1"], true, 5, true, 5, true);
        let items = vec![
            make_text_item("PRIME", 100, 200, 1),
            make_text_item("1,234.56", 100, 200, 1),
        ];
        parser.parse_items(&items);
        let consumed = parser.parse_items(&items[1..]);
        assert_eq!(consumed, 1);
        assert_eq!(parser.amount_parser.value, Some(-1234.56));
    }

    #[test]
    fn test_primer_x1_fail() {
        let mut parser = PrimedAmountParser::new(&["PRIME"], &["format1"], true, 1, false, 0, false);
        let items = vec![
            make_text_item("PRIME", 100, 200, 1),
            make_text_item("1,234.56", 105, 200, 1),
        ];
        parser.parse_items(&items);
        let consumed = parser.parse_items(&items[1..]);
        assert_eq!(consumed, 0);
        assert!(parser.amount_parser.value.is_none());
    }

    #[test]
    fn test_primer_y1_fail() {
        let mut parser = PrimedAmountParser::new(&["PRIME"], &["format1"], false, 0, true, 1, false);
        let items = vec![
            make_text_item("PRIME", 100, 200, 1),
            make_text_item("1,234.56", 100, 205, 1),
        ];
        parser.parse_items(&items);
        let consumed = parser.parse_items(&items[1..]);
        assert_eq!(consumed, 0);
        assert!(parser.amount_parser.value.is_none());
    }

    #[test]
    fn test_primer_page_fail() {
        let mut parser = PrimedAmountParser::new(&["PRIME"], &["format1"], false, 0, false, 0, false);
        let items = vec![
            make_text_item("PRIME", 100, 200, 1),
            make_text_item("1,234.56", 100, 200, 2),
        ];
        parser.parse_items(&items);
        let consumed = parser.parse_items(&items[1..]);
        assert_eq!(consumed, 0);
        assert!(parser.amount_parser.value.is_none());
    }

    #[test]
    fn test_no_items() {
        let mut parser = PrimedAmountParser::new(&["PRIME"], &["format1"], true, 5, true, 5, false);
        let items: Vec<TextItem> = vec![];
        let consumed = parser.parse_items(&items);
        assert_eq!(consumed, 0);
        assert!(!parser.primer_parser.primed);
        assert!(parser.amount_parser.value.is_none());
    }

    #[test]
    fn test_amount_already_set() {
        let mut parser = PrimedAmountParser::new(&["PRIME"], &["format1"], true, 5, true, 5, false);
        let items = vec![
            make_text_item("PRIME", 100, 200, 1),
            make_text_item("1,234.56", 100, 200, 1),
        ];
        parser.parse_items(&items);
        parser.parse_items(&items[1..]);
        // Try parsing again, should return 0 since value is already set
        let consumed = parser.parse_items(&items[1..]);
        assert_eq!(consumed, 0);
    }
}
