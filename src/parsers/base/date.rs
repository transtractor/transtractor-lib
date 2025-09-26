use crate::formats::date::MultiDateFormatParser;
use crate::structs::TextItem;

/// DateParser: parses date strings using multiple date formats.
pub struct DateParser {
    /// The current parsed UTC timestamp (milliseconds since epoch)
    pub value: i64,
    /// Parser is ready to scan text for dates
    pub primed: bool,
    /// Parser has found and set valid date
    pub ready: bool,
    /// Number of singular text items consumed when date parsed
    pub consumed: usize,
    /// The last successfully parsed text item
    pub text_item: TextItem,
    /// The multi-format date parser
    pub parser: MultiDateFormatParser,
    /// The maximum number of items among all formats
    pub parser_max_items: usize,
}

impl DateParser {
    /// Create a new DateParser with specified format names
    pub fn new(format_names: &[&str]) -> Self {
        let parser = MultiDateFormatParser::new(format_names);
        let parser_max_items = parser.max_items();
        DateParser {
            value: 0,
            primed: false,
            ready: false,
            consumed: 0,
            text_item: TextItem::default(),
            parser,
            parser_max_items,
        }
    }

    /// Reset the parser state
    pub fn reset(&mut self) {
        self.value = 0;
        self.ready = false;
        self.consumed = 0;
        self.text_item = TextItem::default();
    }

    /// Returns true if the parser has found a valid date
    pub fn is_ready(&self) -> bool {
        self.ready
    }

    /// Try to parse date from a slice of TextItems, using an optional year_str.
    /// Returns the number of items consumed (0 if not found).
    pub fn parse_items(&mut self, items: &[TextItem], year_str: &str) -> usize {
        if !self.primed || self.ready || items.is_empty() {
            return 0;
        }
        let max = usize::min(self.parser_max_items, items.len());
        for i in (1..=max).rev() {
            let joined = items[0..i]
                .iter()
                .map(|t| t.text.as_str())
                .collect::<Vec<_>>()
                .join(" ");
            if let Some(val) = self.parser.parse(&joined, year_str) {
                self.value = val;
                self.ready = true;
                self.consumed = i;
                self.text_item = items[0..i].last().cloned().unwrap_or_else(TextItem::default);
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
        parser.primed = true;
        let items = vec![make_text_item("24 mar")];
        let consumed = parser.parse_items(&items, "2023");
        assert_eq!(consumed, 1);
        assert!(parser.is_ready());
        assert!(parser.value > 0);
    }

    #[test]
    fn test_parse_multiple_items_format2() {
        let mut parser = DateParser::new(&["format2"]);
        parser.primed = true;
        let items = vec![make_text_item("24"), make_text_item("march"), make_text_item("2020")];
        let consumed = parser.parse_items(&items, "");
        assert_eq!(consumed, 3);
        assert!(parser.is_ready());
        assert!(parser.value > 0);
    }

    #[test]
    fn test_no_match() {
        let mut parser = DateParser::new(&["format1"]);
        parser.primed = true;
        let items = vec![make_text_item("foo")];
        let consumed = parser.parse_items(&items, "2023");
        assert_eq!(consumed, 0);
        assert!(!parser.is_ready());
    }

    #[test]
    fn test_reset() {
        let mut parser = DateParser::new(&["format1"]);
        parser.primed = true;
        let items = vec![make_text_item("24 mar")];
        parser.parse_items(&items, "2023");
        assert!(parser.is_ready());
        parser.reset();
        assert!(!parser.is_ready());
        assert_eq!(parser.value, 0);
        assert_eq!(parser.consumed, 0);
    }
}