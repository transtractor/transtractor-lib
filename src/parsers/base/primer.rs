use crate::structs::TextItem;

/// A parser that is primed by matching terms from text items.
pub struct ParserPrimer {
    /// Parser is ready to scan for terms
    pub primed: bool,
    /// The last successfully parsed text item
    pub text_item: TextItem,
    /// The set of terms to match against (lowercase)
    pub terms: Vec<String>,
    /// Number of space-delimited items in the longest term
    pub max_lookahead: usize,
}

impl ParserPrimer {
    /// Create a new ParserPrimer with specified terms
    pub fn new(terms: &[&str]) -> Self {
        let terms_lower: Vec<String> = terms.iter().map(|t| t.to_lowercase()).collect();
        let max_lookahead = terms_lower.iter().map(|t| t.split(' ').count()).max().unwrap_or(0);
        ParserPrimer {
            primed: false,
            text_item: TextItem::default(),
            terms: terms_lower,
            max_lookahead,
        }
    }

    /// Iteratively join text items and attempt to match terms
    /// Returns number of items consumed if successful, else 0
    pub fn parse_items(&mut self, items: &[TextItem]) -> usize {
        if items.is_empty() {
            return 0;
        }
        // Try longest first, then shorter
        let max = usize::min(self.max_lookahead, items.len());
        for i in (1..=max).rev() {
            if let Some(curr_item) = TextItem::from_items(&items[0..i]) {
                let curr_text = curr_item.text.to_lowercase();
                if self.terms.iter().any(|t| t == &curr_text) {
                    self.text_item = curr_item;
                    self.primed = true;
                    return i;
                }
            }
        }
        0
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
    fn test_single_term_match() {
        let mut parser = ParserPrimer::new(&["hello"]);
        let items = vec![make_text_item("hello")];
        let consumed = parser.parse_items(&items);
        assert_eq!(consumed, 1);
        assert!(parser.primed);
        assert_eq!(parser.text_item.text, "hello");
    }

    #[test]
    fn test_multi_word_term_match() {
        let mut parser = ParserPrimer::new(&["hello world"]);
        let items = vec![make_text_item("hello"), make_text_item("world")];
        let consumed = parser.parse_items(&items);
        assert_eq!(consumed, 2);
        assert!(parser.primed);
        assert_eq!(parser.text_item.text, "hello world");
    }

    #[test]
    fn test_no_match() {
        let mut parser = ParserPrimer::new(&["foo"]);
        let items = vec![make_text_item("bar")];
        let consumed = parser.parse_items(&items);
        assert_eq!(consumed, 0);
        assert!(!parser.primed);
    }

    #[test]
    fn test_case_insensitive_match() {
        let mut parser = ParserPrimer::new(&["Hello"]);
        let items = vec![make_text_item("HELLO")];
        let consumed = parser.parse_items(&items);
        assert_eq!(consumed, 1);
        assert!(parser.primed);
    }

    #[test]
    fn test_reset() {
        let mut parser = ParserPrimer::new(&["hello"]);
        let items = vec![make_text_item("hello")];
        parser.parse_items(&items);
        assert!(parser.primed);
        parser.primed = false;
        parser.text_item = TextItem::default();
        assert!(!parser.primed);
        assert_eq!(parser.text_item.text, "");
    }

    #[test]
    fn test_empty_items() {
        let mut parser = ParserPrimer::new(&["hello"]);
        let items: Vec<TextItem> = vec![];
        let consumed = parser.parse_items(&items);
        assert_eq!(consumed, 0);
        assert!(!parser.primed);
    }
}