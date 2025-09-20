use crate::formats::MultiAmountFormatParser;
use crate::structs::TextItem;

pub struct AmountParser {
    /// The current amount that has been successfully parsed
    pub value: f64,
    /// Parser is ready to scan text for amounts
    pub primed: bool,
    /// Parser has found and set valid amount
    pub ready: bool,
    /// Number of singular text items consumed to generate current amount
    pub consumed: usize,
    /// Dispatcher for multiple amount formats
    pub parser: MultiAmountFormatParser,
    /// A copy of the last successfully parsed text item
    pub text_item: TextItem,
}

impl AmountParser {
    /// Create a new AmountParser with specified format names
    pub fn new(format_names: &[&str]) -> Self {
        AmountParser {
            value: 0.0,
            primed: false,
            ready: false,
            consumed: 0,
            parser: MultiAmountFormatParser::new(format_names),
            text_item: TextItem::default(),
        }
    }

    /// Reset the parser state
    pub fn reset(&mut self) {
        self.value = 0.0;
        self.primed = false;
        self.ready = false;
        self.consumed = 0;
        self.text_item = TextItem::default();
    }

    /// Iteratively join text items and attempt to parse amounts
    /// Returns number of items consumed if successful, else 0
    pub fn parse_items(&mut self, items: &[TextItem]) -> usize {
        // Ignore parser if already set or not primed
        if !self.primed || self.ready || items.is_empty() {
            return 0;
        }
        // Try longest first, then shorter
        for i in (1..=items.len()).rev() {
            if let Some(curr_item) = TextItem::from_items(&items[0..i]) {
                if let Some(val) = self.parser.parse(&curr_item.text) {
                    self.value = val;
                    self.ready = true;
                    self.consumed = i;
                    self.text_item = curr_item;
                    return i;
                }
            }
        }
        0
    }

    /// Invert the sign of the parsed amount
    pub fn invert(&mut self) {
        self.value = -self.value;
    }

    /// Check if the parser has a valid amount
    pub fn is_ready(&self) -> bool {
        self.ready
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::TextItem;

    fn make_text_item(text: &str) -> TextItem {
        TextItem {
            text: text.to_string(),
            x1: 0,
            y1: 0,
            x2: 0,
            y2: 0,
            page: 1,
        }
    }

    #[test]
    fn test_parse_single_item() {
        let mut parser = AmountParser::new(&["format1"]);
        parser.primed = true;
        let items = vec![make_text_item("1,234.56")];
        let consumed = parser.parse_items(&items);
        assert_eq!(consumed, 1);
        assert_eq!(parser.value, 1234.56);
        assert!(parser.is_ready());
    }

    #[test]
    fn test_parse_longest_first() {
        let mut parser = AmountParser::new(&["format1"]);
        parser.primed = true;
        let items = vec![
            make_text_item("1,234"),
            make_text_item(".56"),
        ];
        // "1,234 .56" is not a valid format1, so should fail
        let consumed = parser.parse_items(&items);
        assert_eq!(consumed, 0);
        assert!(!parser.is_ready());
    }

    #[test]
    fn test_parse_longest_first_successfully() {
        let mut parser = AmountParser::new(&["format4"]);
        parser.primed = true;
        let items = vec![
            make_text_item("1,234.56"),
            make_text_item("DR"),
        ];
        // "1,234 DR" is a valid format4, so should succeed
        let consumed = parser.parse_items(&items);
        assert_eq!(consumed, 2);
        assert!(parser.is_ready());
    }

    #[test]
    fn test_parse_multiple_formats() {
        let mut parser = AmountParser::new(&["format2", "format1"]);
        parser.primed = true;
        let items = vec![make_text_item("-$1,234.56")];
        let consumed = parser.parse_items(&items);
        assert_eq!(consumed, 1);
        assert_eq!(parser.value, -1234.56);
        assert!(parser.is_ready());
    }

    #[test]
    fn test_reset() {
        let mut parser = AmountParser::new(&["format1"]);
        parser.primed = true;
        let items = vec![make_text_item("1,234.56")];
        parser.parse_items(&items);
        assert!(parser.is_ready());
        parser.reset();
        assert_eq!(parser.value, 0.0);
        assert!(!parser.is_ready());
        assert_eq!(parser.consumed, 0);
    }

    #[test]
    fn test_invert() {
        let mut parser = AmountParser::new(&["format1"]);
        parser.primed = true;
        let items = vec![make_text_item("1,234.56")];
        parser.parse_items(&items);
        parser.invert();
        assert_eq!(parser.value, -1234.56);
    }

    #[test]
    fn test_not_primed() {
        let mut parser = AmountParser::new(&["format1"]);
        let items = vec![make_text_item("1,234.56")];
        let consumed = parser.parse_items(&items);
        assert_eq!(consumed, 0);
        assert!(!parser.is_ready());
    }

    #[test]
    fn test_empty_items() {
        let mut parser = AmountParser::new(&["format1"]);
        parser.primed = true;
        let items: Vec<TextItem> = vec![];
        let consumed = parser.parse_items(&items);
        assert_eq!(consumed, 0);
        assert!(!parser.is_ready());
    }
}