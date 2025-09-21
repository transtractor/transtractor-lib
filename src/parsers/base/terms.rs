use crate::structs::TextItem;

pub struct TermsParser {
    /// Parser is ready to scan for terms
    pub primed: bool,
    /// Parser has already successfully parsed a term
    pub ready: bool,
    /// Number of singular text items consumed when terms parsed
    pub consumed: usize,
    /// The last successfully parsed text item
    pub text_item: TextItem,
    /// The set of terms to match against (lowercase)
    pub terms: Vec<String>,
    /// Number of space-delimited items in the longest term
    pub parser_max_items: usize,
}

impl TermsParser {
    /// Create a new TermsParser with specified terms
    pub fn new(terms: &[&str]) -> Self {
        let terms_lower: Vec<String> = terms.iter().map(|t| t.to_lowercase()).collect();
        let parser_max_items = terms_lower.iter().map(|t| t.split(' ').count()).max().unwrap_or(0);
        TermsParser {
            primed: false,
            ready: false,
            consumed: 0,
            text_item: TextItem::default(),
            terms: terms_lower,
            parser_max_items,
        }
    }

    /// Reset the parser state
    pub fn reset(&mut self) {
        self.primed = false;
        self.ready = false;
        self.consumed = 0;
        self.text_item = TextItem::default();
    }

    /// Set the parser as primed to scan for terms
    pub fn prime(&mut self) {
        self.primed = true;
    }

    /// Check if parser has successfully parsed a term
    pub fn is_ready(&self) -> bool {
        self.ready
    }

    /// Iteratively join text items and attempt to match terms
    /// Returns number of items consumed if successful, else 0
    pub fn parse_items(&mut self, items: &[TextItem]) -> usize {
        // Ignore parser if already set or not primed
        if !self.primed || self.ready || items.is_empty() {
            return 0;
        }
        // Try longest first, then shorter
        let max = usize::min(self.parser_max_items, items.len());
        for i in (1..=max).rev() {
            if let Some(curr_item) = TextItem::from_items(&items[0..i]) {
                let curr_text = curr_item.text.to_lowercase();
                if self.terms.iter().any(|t| t == &curr_text) {
                    self.ready = true;
                    self.consumed = i;
                    self.text_item = curr_item;
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
        let mut parser = TermsParser::new(&["hello"]);
        parser.prime();
        let items = vec![make_text_item("hello")];
        let consumed = parser.parse_items(&items);
        assert_eq!(consumed, 1);
        assert!(parser.is_ready());
        assert_eq!(parser.text_item.text, "hello");
    }

    #[test]
    fn test_multi_word_term_match() {
        let mut parser = TermsParser::new(&["hello world"]);
        parser.prime();
        let items = vec![make_text_item("hello"), make_text_item("world")];
        let consumed = parser.parse_items(&items);
        assert_eq!(consumed, 2);
        assert!(parser.is_ready());
        assert_eq!(parser.text_item.text, "hello world");
    }

    #[test]
    fn test_no_match() {
        let mut parser = TermsParser::new(&["foo"]);
        parser.prime();
        let items = vec![make_text_item("bar")];
        let consumed = parser.parse_items(&items);
        assert_eq!(consumed, 0);
        assert!(!parser.is_ready());
    }

    #[test]
    fn test_case_insensitive_match() {
        let mut parser = TermsParser::new(&["Hello"]);
        parser.prime();
        let items = vec![make_text_item("HELLO")];
        let consumed = parser.parse_items(&items);
        assert_eq!(consumed, 1);
        assert!(parser.is_ready());
    }

    #[test]
    fn test_reset() {
        let mut parser = TermsParser::new(&["hello"]);
        parser.prime();
        let items = vec![make_text_item("hello")];
        parser.parse_items(&items);
        assert!(parser.is_ready());
        parser.reset();
        assert!(!parser.is_ready());
        assert_eq!(parser.consumed, 0);
        assert_eq!(parser.text_item.text, "");
    }

    #[test]
    fn test_not_primed() {
        let mut parser = TermsParser::new(&["hello"]);
        let items = vec![make_text_item("hello")];
        let consumed = parser.parse_items(&items);
        assert_eq!(consumed, 0);
        assert!(!parser.is_ready());
    }

    #[test]
    fn test_empty_items() {
        let mut parser = TermsParser::new(&["hello"]);
        parser.prime();
        let items: Vec<TextItem> = vec![];
        let consumed = parser.parse_items(&items);
        assert_eq!(consumed, 0);
        assert!(!parser.is_ready());
    }
}

