use crate::structs::TextItem;
use crate::parsers::base::DateParser;
use crate::parsers::base::ParserPrimer;

pub struct PrimedDateParser {
    primer_parser: ParserPrimer,
    date_parser: DateParser,
    same_x1: bool,
    x1_tol: i32,
    same_y1: bool,
    y1_tol: i32,
    invert: bool,
}

impl PrimedDateParser {
    pub fn new(
        primer_terms: &[&str],
        date_formats: &[&str],
        same_x1: bool,
        x1_tol: i32,
        same_y1: bool,
        y1_tol: i32,
        invert: bool,
    ) -> Self {
        Self {
            primer_parser: ParserPrimer::new(primer_terms),
            date_parser: DateParser::new(date_formats),
            same_x1,
            x1_tol,
            same_y1,
            y1_tol,
            invert,
        }
    }

    pub fn parse_items(&mut self, items: &[TextItem]) -> usize {
        if items.is_empty() {
            return 0;
        }

        // Return if value already set
        if self.date_parser.value.is_some() {
            return 0;
        }

        // Try to prime (if not already primed)
        if !self.primer_parser.primed {
            return self.primer_parser.parse_items(items);
        }

        // Primer is primed, look for date
        let consumed = self.date_parser.parse_items(items, "");
        if consumed == 0 {
            return 0; // No date found
        }

        // Both primer and date found, check conditions
        let date_item = self.date_parser.text_item.as_ref().unwrap();
        let primer_item = self.primer_parser.text_item.as_ref().unwrap();

        // Check x1, y1 and page conditions
        let x1_ok = if self.same_x1 {
            (date_item.x1 - primer_item.x1).abs() <= self.x1_tol
        } else {
            true
        };
        let y1_ok = if self.same_y1 {
            (date_item.y1 - primer_item.y1).abs() <= self.y1_tol
        } else {
            true
        };
        let page_ok = date_item.page == primer_item.page;

        // Return 0 if any condition fails
        if !x1_ok || !y1_ok || !page_ok {
            // Reset date parser state
            self.date_parser.reset();
            return 0;
        }

        // All conditions met, finalize
        if self.invert {
            if let Some(val) = self.date_parser.value {
                self.date_parser.value = Some(-val);
            }
        }
        consumed
    }

    pub fn value(&self) -> Option<i64> {
        self.date_parser.value
    }

    /// Whether the primer term has been matched
    pub fn is_primed(&self) -> bool {
        self.primer_parser.primed
    }

    /// Get the highest lookahead between primer and date parsers
    pub fn get_max_lookahead(&self) -> usize {
        self.primer_parser.max_lookahead
            .max(self.date_parser.max_lookahead)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::TextItem;

    fn make_text_item(text: &str, x1: i32, y1: i32, page: i32) -> TextItem {
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
    fn test_primer_and_date_success() {
        let mut parser = PrimedDateParser::new(&["DATE"], &["format2"], true, 5, true, 5, false);
        let items = vec![
            make_text_item("DATE", 100, 200, 1),
            make_text_item("24 march 2020", 102, 202, 1),
        ];
        // First call primes the parser
        let consumed_primer = parser.parse_items(&items);
        assert_eq!(consumed_primer, 1);
        assert!(parser.primer_parser.primed);
        assert!(parser.date_parser.value.is_none());

        // Second call parses the date
        let consumed_date = parser.parse_items(&items[1..]);
        assert_eq!(consumed_date, 1);
        assert!(parser.date_parser.value.is_some());
    }

    #[test]
    fn test_primer_and_date_invert() {
        let mut parser = PrimedDateParser::new(&["DATE"], &["format2"], true, 5, true, 5, true);
        let items = vec![
            make_text_item("DATE", 100, 200, 1),
            make_text_item("24 march 2020", 100, 200, 1),
        ];
        parser.parse_items(&items);
        let consumed = parser.parse_items(&items[1..]);
        assert_eq!(consumed, 1);
        assert!(parser.date_parser.value.is_some());
        assert!(parser.date_parser.value.unwrap() < 0);
    }

    #[test]
    fn test_primer_x1_fail() {
        let mut parser = PrimedDateParser::new(&["DATE"], &["format2"], true, 1, false, 0, false);
        let items = vec![
            make_text_item("DATE", 100, 200, 1),
            make_text_item("24 march 2020", 105, 200, 1),
        ];
        parser.parse_items(&items);
        let consumed = parser.parse_items(&items[1..]);
        assert_eq!(consumed, 0);
        assert!(parser.date_parser.value.is_none());
    }

    #[test]
    fn test_primer_y1_fail() {
        let mut parser = PrimedDateParser::new(&["DATE"], &["format2"], false, 0, true, 1, false);
        let items = vec![
            make_text_item("DATE", 100, 200, 1),
            make_text_item("24 march 2020", 100, 205, 1),
        ];
        parser.parse_items(&items);
        let consumed = parser.parse_items(&items[1..]);
        assert_eq!(consumed, 0);
        assert!(parser.date_parser.value.is_none());
    }

    #[test]
    fn test_primer_page_fail() {
        let mut parser = PrimedDateParser::new(&["DATE"], &["format2"], false, 0, false, 0, false);
        let items = vec![
            make_text_item("DATE", 100, 200, 1),
            make_text_item("24 march 2020", 100, 200, 2),
        ];
        parser.parse_items(&items);
        let consumed = parser.parse_items(&items[1..]);
        assert_eq!(consumed, 0);
        assert!(parser.date_parser.value.is_none());
    }

    #[test]
    fn test_no_items() {
        let mut parser = PrimedDateParser::new(&["DATE"], &["format2"], true, 5, true, 5, false);
        let items: Vec<TextItem> = vec![];
        let consumed = parser.parse_items(&items);
        assert_eq!(consumed, 0);
        assert!(!parser.primer_parser.primed);
        assert!(parser.date_parser.value.is_none());
    }

    #[test]
    fn test_date_already_set() {
        let mut parser = PrimedDateParser::new(&["DATE"], &["format2"], true, 5, true, 5, false);
        let items = vec![
            make_text_item("DATE", 100, 200, 1),
            make_text_item("24 march 2020", 100, 200, 1),
        ];
        parser.parse_items(&items);
        parser.parse_items(&items[1..]);
        // Try parsing again, should return 0 since value is already set
        let consumed = parser.parse_items(&items[1..]);
        assert_eq!(consumed, 0);
    }
}