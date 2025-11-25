use crate::structs::TextItem;
use regex::Regex;

/// A parser for reading values based on regex patterns.
pub struct ValueParser {
    /// The current account number that has been successfully parsed
    pub value: Option<String>,
    /// The last successfully parsed text item
    pub text_item: Option<TextItem>,
    /// TRegex patterns to match against
    pub patterns: Vec<Regex>,
    /// Number of space-delimited items in the longest regex pattern
    pub max_lookahead: usize,
}

impl ValueParser {
    /// Create a new ValueParser with specified regex patterns.
    /// The max_lookahead is automatically calculated from the patterns by counting
    /// the number of whitespace-separated terms each pattern expects.
    pub fn new(patterns: &[Regex]) -> Self {
        let max_lookahead = Self::calculate_max_lookahead(patterns);
        ValueParser {
            value: None,
            text_item: None,
            patterns: patterns.to_vec(),
            max_lookahead,
        }
    }

    /// Calculate the maximum lookahead from regex patterns by estimating
    /// the number of whitespace-separated terms expected.
    fn calculate_max_lookahead(patterns: &[Regex]) -> usize {
        patterns
            .iter()
            .map(|p| {
                let pattern_str = p.as_str();
                // Count only whitespace separators: \s, \s+, \s*, literal space
                let separator_count =
                    pattern_str.matches(r"\s").count() + pattern_str.matches(" ").count();
                // Add 1 because N separators means N+1 tokens
                // Use at least 1 as minimum lookahead
                (separator_count + 1).max(1)
            })
            .max()
            .unwrap_or(1)
    }

    /// Get text item, raise error if none
    pub fn text_item(&self) -> &TextItem {
        self.text_item.as_ref().expect("No text item available")
    }

    /// Iteratively join text items and attempt to match regex patterns
    /// Returns number of items consumed if successful, else 0
    pub fn parse_items(&mut self, items: &[TextItem]) -> usize {
        if items.is_empty() {
            return 0;
        }
        // Try longest first, then shorter
        let max = usize::min(self.max_lookahead, items.len());
        for i in (1..=max).rev() {
            if let Some(curr_item) = TextItem::from_items(&items[0..i]) {
                let curr_text = &curr_item.text;
                if self.patterns.iter().any(|p| p.is_match(curr_text)) {
                    self.value = Some(curr_text.clone());
                    self.text_item = Some(curr_item);
                    return i;
                }
            }
        }
        0
    }

    /// Reset the parser
    pub fn reset(&mut self) {
        self.value = None;
        self.text_item = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_item(text: &str) -> TextItem {
        TextItem::new(text.to_string(), 0, 0, 100, 12, 1)
    }

    #[test]
    fn test_new_parser_not_primed() {
        let patterns = vec![Regex::new(r"\b\d{4}\b").unwrap()];
        let parser = ValueParser::new(&patterns);
        assert!(parser.value.is_none());
        assert!(parser.text_item.is_none());
        assert_eq!(parser.max_lookahead, 1); // Single token pattern
    }

    #[test]
    fn test_parse_single_item_match() {
        let patterns = vec![Regex::new(r"\b\d{4}\b").unwrap()];
        let mut parser = ValueParser::new(&patterns);
        let items = vec![create_test_item("1234")];

        let consumed = parser.parse_items(&items);
        
        assert_eq!(consumed, 1);
        assert_eq!(parser.value, Some("1234".to_string()));
        assert_eq!(parser.text_item().text, "1234");
    }    #[test]
    fn test_parse_single_item_no_match() {
        let patterns = vec![Regex::new(r"\b\d{4}\b").unwrap()];
        let mut parser = ValueParser::new(&patterns);
        let items = vec![create_test_item("ABC")];

        let consumed = parser.parse_items(&items);
        
        assert_eq!(consumed, 0);
        assert!(parser.value.is_none());
        assert!(parser.text_item.is_none());
    }    #[test]
    fn test_parse_multiple_items_joined() {
        // Pattern expects numbers separated by spaces and hyphens (TextItem::from_items adds spaces)
        let patterns = vec![Regex::new(r"\d+\s+-\s+\d+\s+-\s+\d+").unwrap()];
        let mut parser = ValueParser::new(&patterns);
        let items = vec![
            create_test_item("082"),
            create_test_item("-"),
            create_test_item("738"),
            create_test_item("-"),
            create_test_item("12345678"),
        ];

        let consumed = parser.parse_items(&items);
        
        assert_eq!(consumed, 5);
        assert_eq!(parser.value, Some("082 - 738 - 12345678".to_string()));
        assert_eq!(parser.text_item().text, "082 - 738 - 12345678");
    }    #[test]
    fn test_parse_space_separated_numbers() {
        // Pattern expects three numbers separated by spaces
        let patterns = vec![Regex::new(r"\b\d+\s+\d+\s+\d+\b").unwrap()];
        let mut parser = ValueParser::new(&patterns);
        let items = vec![
            create_test_item("1234"),
            create_test_item("5678"),
            create_test_item("9012"),
        ];

        let consumed = parser.parse_items(&items);
        
        assert_eq!(consumed, 3);
        assert_eq!(parser.value, Some("1234 5678 9012".to_string()));
        assert_eq!(parser.text_item().text, "1234 5678 9012");
    }    #[test]
    fn test_parse_longest_match_first() {
        // Two patterns: one for hyphenated pair (with spaces), one for single number
        let patterns = vec![
            Regex::new(r"\b\d+\s+-\s+\d+\b").unwrap(), // Matches "123 - 456"
            Regex::new(r"\b\d+\b").unwrap(),           // Matches "123"
        ];
        let mut parser = ValueParser::new(&patterns);
        let items = vec![
            create_test_item("123"),
            create_test_item("-"),
            create_test_item("456"),
        ];

        let consumed = parser.parse_items(&items);

        // Should match the longer pattern (all 3 items)
        assert_eq!(consumed, 3);
        assert_eq!(parser.text_item().text, "123 - 456");
    }

    #[test]
    fn test_parse_respects_max_lookahead() {
        // Pattern with 2 \s+ means 3 tokens expected
        let patterns = vec![Regex::new(r"\b\d+\s+\d+\s+\d+\b").unwrap()];
        let mut parser = ValueParser::new(&patterns);
        let items = vec![
            create_test_item("123"),
            create_test_item("456"),
            create_test_item("789"),
        ];

        // Auto-calculated lookahead should allow matching all 3 items
        let consumed = parser.parse_items(&items);
        assert_eq!(consumed, 3);
        assert_eq!(parser.value, Some("123 456 789".to_string()));
    }

    #[test]
    fn test_parse_empty_items() {
        let patterns = vec![Regex::new(r"\b\d+\b").unwrap()];
        let mut parser = ValueParser::new(&patterns);
        let items: Vec<TextItem> = vec![];

        let consumed = parser.parse_items(&items);
        
        assert_eq!(consumed, 0);
        assert!(parser.value.is_none());
    }    #[test]
    fn test_reset() {
        let patterns = vec![Regex::new(r"\b\d{4}\b").unwrap()];
        let mut parser = ValueParser::new(&patterns);
        let items = vec![create_test_item("1234")];

        parser.parse_items(&items);
        assert!(parser.value.is_some());
        assert!(parser.text_item.is_some());
        
        parser.reset();
        assert!(parser.value.is_none());
        assert!(parser.text_item.is_none());
    }    #[test]
    fn test_multiple_patterns() {
        let patterns = vec![
            Regex::new(r"\b\d{4}\s\d{4}\s\d{4}\s\d{4}\b").unwrap(), // 4-part space-separated
            Regex::new(r"\d+\s+-\s+\d+\s+-\s+\d+").unwrap(),        // hyphen with spaces
            Regex::new(r"\b\d{10,}\b").unwrap(),                    // 10+ digits
        ];
        let mut parser = ValueParser::new(&patterns);

        // Test first pattern
        let items1 = vec![
            create_test_item("1234"),
            create_test_item("5678"),
            create_test_item("9012"),
            create_test_item("3456"),
        ];
        assert_eq!(parser.parse_items(&items1), 4);
        assert_eq!(parser.text_item().text, "1234 5678 9012 3456");

        parser.reset();

        // Test second pattern
        let items2 = vec![
            create_test_item("082"),
            create_test_item("-"),
            create_test_item("738"),
            create_test_item("-"),
            create_test_item("12345678"),
        ];
        assert_eq!(parser.parse_items(&items2), 5);
        assert_eq!(parser.text_item().text, "082 - 738 - 12345678");

        parser.reset();

        // Test third pattern
        let items3 = vec![create_test_item("1234567890123")];
        assert_eq!(parser.parse_items(&items3), 1);
        assert_eq!(parser.text_item().text, "1234567890123");
    }

    #[test]
    #[should_panic(expected = "No text item available")]
    fn test_text_item_panics_when_none() {
        let patterns = vec![Regex::new(r"\b\d+\b").unwrap()];
        let parser = ValueParser::new(&patterns);
        let _ = parser.text_item(); // Should panic
    }
}
