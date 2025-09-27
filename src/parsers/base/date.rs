use crate::formats::date::MultiDateFormatParser;
use crate::structs::TextItem;

/// DateParser: parses date strings using multiple date formats.
pub struct DateParser {
    /// The current parsed UTC timestamp (milliseconds since epoch)
    pub value: Option<i64>,
    /// Dispatcher for multiple date formats
    pub parser: MultiDateFormatParser,
    /// Maximum number of space-delimited items in the selected formats
    pub max_lookahead: usize,
    /// A copy of the last successfully parsed text item (merged text)
    pub text_item: Option<TextItem>,
}

impl DateParser {
    /// Create a new DateParser with specified format names
    pub fn new(format_names: &[&str]) -> Self {
        let parser = MultiDateFormatParser::new(format_names);
        let max_lookahead = parser.max_items();
        DateParser {
            value: None,
            parser,
            max_lookahead,
            text_item: None,
        }
    }

    /// Reset the parser state
    pub fn reset(&mut self) {
        self.value = None;
        self.text_item = None;
    }

    /// Iteratively join text items and attempt to parse dates
    /// Returns number of items consumed if successful, else 0
    pub fn parse_items(&mut self, items: &[TextItem], year_str: &str) -> usize {
        if items.is_empty() {
            return 0;
        }
        // Try longest first, then shorter
        let max = usize::min(self.max_lookahead, items.len());
        for i in (1..=max).rev() {
            let merged = items[0..i]
                .iter()
                .map(|t| t.text.as_str())
                .collect::<Vec<_>>()
                .join(" ");
            if let Some(val) = self.parser.parse(&merged, year_str) {
                self.value = Some(val);
                self.text_item = Some(TextItem {
                    text: merged,
                    ..items[0].clone()
                });
                return i;
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
    fn test_parse_single_item_format1() {
        let mut parser = DateParser::new(&["format1"]);
        let items = vec![make_text_item("24 mar")];
        let consumed = parser.parse_items(&items, "2023");
        assert_eq!(consumed, 1);
        assert!(parser.value.is_some());
        assert_eq!(parser.text_item.as_ref().unwrap().text, "24 mar");
    }

    #[test]
    fn test_parse_multiple_items_format2() {
        let mut parser = DateParser::new(&["format2"]);
        let items = vec![make_text_item("24"), make_text_item("march"), make_text_item("2020")];
        let consumed = parser.parse_items(&items, "");
        assert_eq!(consumed, 3);
        assert!(parser.value.is_some());
        assert_eq!(parser.text_item.as_ref().unwrap().text, "24 march 2020");
    }

    #[test]
    fn test_no_match() {
        let mut parser = DateParser::new(&["format1"]);
        let items = vec![make_text_item("foo")];
        let consumed = parser.parse_items(&items, "2023");
        assert_eq!(consumed, 0);
        assert!(parser.value.is_none());
        assert!(parser.text_item.is_none());
    }

    #[test]
    fn test_reset() {
        let mut parser = DateParser::new(&["format1"]);
        let items = vec![make_text_item("24 mar")];
        parser.parse_items(&items, "2023");
        assert!(parser.value.is_some());
        parser.reset();
        assert!(parser.value.is_none());
        assert!(parser.text_item.is_none());
    }
}