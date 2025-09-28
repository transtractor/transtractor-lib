use crate::formats::MultiAmountFormatParser;
use crate::structs::TextItem;

pub struct AmountParser {
    /// The current amount that has been successfully parsed
    pub value: Option<f64>,
    /// Dispatcher for multiple amount formats
    pub parser: MultiAmountFormatParser,
    /// Maximum number of space-delimited items in the selected formats
    pub max_lookahead: usize,
    /// A copy of the last successfully parsed text item
    pub text_item: Option<TextItem>,
}

impl AmountParser {
    /// Create a new AmountParser with specified format names
    pub fn new(format_names: &[&str]) -> Self {
        let parser = MultiAmountFormatParser::new(format_names);
        let max_lookahead = parser.max_items();
        AmountParser {
            value: None,
            parser,
            max_lookahead,
            text_item: None,
        }
    }

    /// Get text item, raise error if none
    pub fn text_item(&self) -> &TextItem {
        self.text_item.as_ref().expect("No text item available")
    }

    /// Reset the parser state
    pub fn reset(&mut self) {
        self.value = None;
        self.text_item = None;
    }

    /// Iteratively join text items and attempt to parse amounts
    /// Returns number of items consumed if successful, else 0
    pub fn parse_items(&mut self, items: &[TextItem]) -> usize {
        if items.is_empty() {
            return 0;
        }
        // Try longest first, then shorter
        let max = usize::min(self.max_lookahead, items.len());
        for i in (1..=max).rev() {
            if let Some(curr_item) = TextItem::from_items(&items[0..i]) {
                if let Some(val) = self.parser.parse(&curr_item.text) {
                    self.value = Some(val);
                    self.text_item = Some(curr_item);
                    return i;
                }
            }
        }
        0
    }

    /// Invert the sign of the parsed amount
    pub fn invert(&mut self) {
        if let Some(val) = self.value {
            self.value = Some(-val);
        } else {
            panic!("Cannot invert: no value has been parsed");
        }
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
        let items = vec![make_text_item("1,234.56")];
        let consumed = parser.parse_items(&items);
        assert_eq!(consumed, 1);
        assert_eq!(parser.value, Some(1234.56));
        assert_eq!(parser.text_item.as_ref().unwrap().text, "1,234.56");
    }

    #[test]
    fn test_parse_longest_first() {
        let mut parser = AmountParser::new(&["format1"]);
        let items = vec![
            make_text_item("1,234"),
            make_text_item(".56"),
        ];
        // "1,234 .56" is not a valid format1, so should fail
        let consumed = parser.parse_items(&items);
        assert_eq!(consumed, 0);
        assert_eq!(parser.value, None);
        assert!(parser.text_item.is_none());
    }

    #[test]
    fn test_parse_longest_first_successfully() {
        let mut parser = AmountParser::new(&["format4"]);
        let items = vec![
            make_text_item("1,234.56"),
            make_text_item("DR"),
        ];
        // "1,234.56 DR" is a valid format4, so should succeed
        let consumed = parser.parse_items(&items);
        assert_eq!(consumed, 2);
        assert!(parser.value.is_some());
        assert_eq!(parser.text_item.as_ref().unwrap().text, "1,234.56 DR");
    }

    #[test]
    fn test_parse_multiple_formats() {
        let mut parser = AmountParser::new(&["format2", "format1"]);
        let items = vec![make_text_item("-$1,234.56")];
        let consumed = parser.parse_items(&items);
        assert_eq!(consumed, 1);
        assert_eq!(parser.value, Some(-1234.56));
        assert_eq!(parser.text_item.as_ref().unwrap().text, "-$1,234.56");
    }

    #[test]
    fn test_reset() {
        let mut parser = AmountParser::new(&["format1"]);
        let items = vec![make_text_item("1,234.56")];
        parser.parse_items(&items);
        parser.reset();
        assert_eq!(parser.value, None);
        assert!(parser.text_item.is_none());
    }

    #[test]
    fn test_invert() {
        let mut parser = AmountParser::new(&["format1"]);
        let items = vec![make_text_item("1,234.56")];
        parser.parse_items(&items);
        parser.invert();
        assert_eq!(parser.value, Some(-1234.56));
    }

    #[test]
    fn test_empty_items() {
        let mut parser = AmountParser::new(&["format1"]);
        let items: Vec<TextItem> = vec![];
        let consumed = parser.parse_items(&items);
        assert_eq!(consumed, 0);
        assert_eq!(parser.value, None);
        assert!(parser.text_item.is_none());
    }
}