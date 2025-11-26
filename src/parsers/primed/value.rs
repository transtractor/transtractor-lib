use crate::structs::TextItem;
use crate::parsers::base::ValueParser;
use crate::parsers::base::ParserPrimer;
use regex::Regex;

pub struct PrimedValueParser {
    primer_parser: ParserPrimer,
    value_parser: ValueParser,
    alignment: String,
    alignment_tol: i32,
}

impl PrimedValueParser {
    pub fn new(
        primer_terms: &[&str],
        value_patterns: &[Regex],
        alignment: &str,
        alignment_tol: i32,
    ) -> Self {
        Self {
            primer_parser: ParserPrimer::new(primer_terms),
            value_parser: ValueParser::new(value_patterns),
            alignment: alignment.to_string(),
            alignment_tol,
        }
    }

    pub fn parse_items(&mut self, items: &[TextItem]) -> usize {
        // No items to parse
        if items.is_empty() {
            return 0;
        }

        // Return if value already set
        if self.value_parser.value.is_some() {
            return 0;
        }

        // Primer not primed, or re-prime if term found again
        let consumed_primer = self.primer_parser.parse_items(items);
        if consumed_primer > 0 {
            return consumed_primer;
        }

        // Must be primed to look for account number
        if !self.primer_parser.primed {
            return 0; // Primer not found yet
        }

        // Primer is primed, look for account number
        let consumed = self.value_parser.parse_items(items);
        if consumed == 0 {
            return 0; // No account number found
        }

        // Check coordinate constraints
        let value_item = self.value_parser.text_item();
        let primer_item = self.primer_parser.text_item();
        let valid_alignment = match self.alignment.as_str() {
            "x1" => (value_item.x1 - primer_item.x1).abs() <= self.alignment_tol,
            "x2" => (value_item.x2 - primer_item.x2).abs() <= self.alignment_tol,
            "y1" => (value_item.y1 - primer_item.y1).abs() <= self.alignment_tol,
            "y2" => (value_item.y2 - primer_item.y2).abs() <= self.alignment_tol,
            "" => true, // No alignment constraint
            _ => true, // No alignment constraint
        };
        let page_ok = value_item.page == primer_item.page;
        // Return 0 if any condition fails
        if !valid_alignment || !page_ok {
            // Reset value parser state
            self.value_parser.reset();
            return 0;
        }
        consumed
    }

    pub fn value(&self) -> Option<&str> {
        self.value_parser.value.as_deref()
    }

    /// Whether the primer term has been matched
    pub fn is_primed(&self) -> bool {
        self.primer_parser.primed
    }

    /// Get the highest lookahead between primer and date parsers
    pub fn get_max_lookahead(&self) -> usize {
        self.primer_parser.max_lookahead
            .max(self.value_parser.max_lookahead)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_text_item(text: &str, x1: i32, y1: i32) -> TextItem {
        TextItem {
            text: text.to_string(),
            x1,
            y1,
            x2: x1 + 100,
            y2: y1 + 10,
            page: 1,
        }
    }

    #[test]
    fn test_new_parser_not_primed() {
        let patterns = vec![Regex::new(r"\b\d{4}\b").unwrap()];
        let parser = PrimedValueParser::new(
            &["Account"],
            &patterns,
            "",
            0,
        );
        assert!(parser.value().is_none());
        assert!(!parser.primer_parser.primed);
    }

    #[test]
    fn test_primer_found_first() {
        let patterns = vec![Regex::new(r"\b\d{4}\b").unwrap()];
        let mut parser = PrimedValueParser::new(
            &["Account"],
            &patterns,
            "",
            0,
        );
        
        let items = vec![create_text_item("Account", 100, 100)];
        let consumed = parser.parse_items(&items);
        
        assert_eq!(consumed, 1);
        assert!(parser.primer_parser.primed);
        assert!(parser.value().is_none());
    }

    #[test]
    fn test_account_number_without_primer() {
        let patterns = vec![Regex::new(r"\b\d{4}\b").unwrap()];
        let mut parser = PrimedValueParser::new(
            &["Account"],
            &patterns,
            "",
            0,
        );
        
        let items = vec![create_text_item("1234", 100, 100)];
        let consumed = parser.parse_items(&items);
        
        assert_eq!(consumed, 0);
        assert!(parser.value().is_none());
    }

    #[test]
    fn test_primer_then_account_number() {
        let patterns = vec![Regex::new(r"\b\d{4}\b").unwrap()];
        let mut parser = PrimedValueParser::new(
            &["Account"],
            &patterns,
            "",
            0,
        );
        
        // Prime first
        let items1 = vec![create_text_item("Account", 100, 100)];
        parser.parse_items(&items1);
        
        // Then parse account number
        let items2 = vec![create_text_item("1234", 100, 100)];
        let consumed = parser.parse_items(&items2);
        
        assert_eq!(consumed, 1);
        assert_eq!(parser.value(), Some("1234"));
    }

    #[test]
    fn test_multi_token_account_number() {
        let patterns = vec![Regex::new(r"\b\d+\s+\d+\s+\d+\b").unwrap()];
        let mut parser = PrimedValueParser::new(
            &["Account"],
            &patterns,
            "",
            0,
        );
        
        // Prime first
        let items1 = vec![create_text_item("Account", 100, 100)];
        parser.parse_items(&items1);
        
        // Parse multi-token account number
        let items2 = vec![
            create_text_item("1234", 100, 110),
            create_text_item("5678", 150, 110),
            create_text_item("9012", 200, 110),
        ];
        let consumed = parser.parse_items(&items2);
        
        assert_eq!(consumed, 3);
        assert_eq!(parser.value(), Some("1234 5678 9012"));
    }

    #[test]
    fn test_same_x1_constraint_pass() {
        let patterns = vec![Regex::new(r"\b\d{4}\b").unwrap()];
        let mut parser = PrimedValueParser::new(
            &["Account"],
            &patterns,
            "x1",  // same x1
            5,     // x1_tol
        );
        
        // Prime at x1=100
        let items1 = vec![create_text_item("Account", 100, 100)];
        parser.parse_items(&items1);
        
        // Account number at x1=103 (within tolerance)
        let items2 = vec![create_text_item("1234", 103, 110)];
        let consumed = parser.parse_items(&items2);
        
        assert_eq!(consumed, 1);
        assert_eq!(parser.value(), Some("1234"));
    }

    #[test]
    fn test_same_x1_constraint_fail() {
        let patterns = vec![Regex::new(r"\b\d{4}\b").unwrap()];
        let mut parser = PrimedValueParser::new(
            &["Account"],
            &patterns,
            "x1",  // same x1
            5,     // x1_tol
        );
        
        // Prime at x1=100
        let items1 = vec![create_text_item("Account", 100, 100)];
        parser.parse_items(&items1);
        
        // Account number at x1=150 (outside tolerance)
        let items2 = vec![create_text_item("1234", 150, 110)];
        let consumed = parser.parse_items(&items2);
        
        assert_eq!(consumed, 0);
        assert!(parser.value().is_none());
    }

    #[test]
    fn test_same_y1_constraint_pass() {
        let patterns = vec![Regex::new(r"\b\d{4}\b").unwrap()];
        let mut parser = PrimedValueParser::new(
            &["Account"],
            &patterns,
            "y1",  // same y1
            3,     // y1_tol
        );
        
        // Prime at y1=100
        let items1 = vec![create_text_item("Account", 100, 100)];
        parser.parse_items(&items1);
        
        // Account number at y1=102 (within tolerance)
        let items2 = vec![create_text_item("1234", 200, 102)];
        let consumed = parser.parse_items(&items2);
        
        assert_eq!(consumed, 1);
        assert_eq!(parser.value(), Some("1234"));
    }

    #[test]
    fn test_same_y1_constraint_fail() {
        let patterns = vec![Regex::new(r"\b\d{4}\b").unwrap()];
        let mut parser = PrimedValueParser::new(
            &["Account"],
            &patterns,
            "y1",  // same y1
            3,     // y1_tol
        );
        
        // Prime at y1=100
        let items1 = vec![create_text_item("Account", 100, 100)];
        parser.parse_items(&items1);
        
        // Account number at y1=200 (outside tolerance)
        let items2 = vec![create_text_item("1234", 200, 200)];
        let consumed = parser.parse_items(&items2);
        
        assert_eq!(consumed, 0);
        assert!(parser.value().is_none());
    }

    #[test]
    fn test_already_parsed_returns_zero() {
        let patterns = vec![Regex::new(r"\b\d{4}\b").unwrap()];
        let mut parser = PrimedValueParser::new(
            &["Account"],
            &patterns,
            "",
            0,
        );
        
        // Prime and parse
        let items1 = vec![create_text_item("Account", 100, 100)];
        parser.parse_items(&items1);
        let items2 = vec![create_text_item("1234", 100, 110)];
        parser.parse_items(&items2);
        
        // Try to parse again
        let items3 = vec![create_text_item("5678", 100, 120)];
        let consumed = parser.parse_items(&items3);
        
        assert_eq!(consumed, 0);
        assert_eq!(parser.value(), Some("1234")); // Still has original value
    }

    #[test]
    fn test_re_prime_resets_search() {
        let patterns = vec![Regex::new(r"\b\d{4}\b").unwrap()];
        let mut parser = PrimedValueParser::new(
            &["Account"],
            &patterns,
            "",
            0,
        );
        
        // Prime first time
        let items1 = vec![create_text_item("Account", 100, 100)];
        parser.parse_items(&items1);
        
        // Parse account number
        let items2 = vec![create_text_item("1234", 100, 110)];
        parser.parse_items(&items2);
        assert_eq!(parser.value(), Some("1234"));
        
        // Find primer again - value already set, returns 0
        let items3 = vec![create_text_item("Account", 100, 200)];
        let consumed = parser.parse_items(&items3);
        
        assert_eq!(consumed, 0); // Value already set, no parsing
        assert_eq!(parser.value(), Some("1234")); // Value still set
    }

    #[test]
    fn test_empty_items() {
        let patterns = vec![Regex::new(r"\b\d{4}\b").unwrap()];
        let mut parser = PrimedValueParser::new(
            &["Account"],
            &patterns,
            "",
            0,
        );
        
        let items: Vec<TextItem> = vec![];
        let consumed = parser.parse_items(&items);
        
        assert_eq!(consumed, 0);
    }
}
