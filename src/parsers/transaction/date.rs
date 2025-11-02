use crate::parsers::base::{DateParser, ParserPrimer};
use crate::structs::{ProtoTransaction, StatementConfig, StatementData, TextItem};

pub struct TransactionDateParser {
    pub primed: bool,
    date_parser: DateParser,
    header_primer: ParserPrimer,
    alignment: String,
    x1_range: Vec<i32>,
    x2_range: Vec<i32>,
    x_tol: i32,
    start_date_year_str: String,
}

impl TransactionDateParser {
    pub fn new(config: &StatementConfig) -> Self {
        let primer_terms: Vec<&str> = config
            .transaction_date_headers
            .iter()
            .map(|s| s.as_str())
            .collect();
        let date_formats: Vec<&str> = config
            .transaction_date_formats
            .iter()
            .map(|s| s.as_str())
            .collect();
        let alignment = config.transaction_date_alignment.clone();
        let x_tol = config.transaction_x_tol;
        Self {
            primed: false,
            date_parser: DateParser::new(date_formats.as_slice()),
            header_primer: ParserPrimer::new(primer_terms.as_slice()),
            alignment,
            x_tol,
            x1_range: vec![0, 10000],
            x2_range: vec![0, 10000],
            start_date_year_str: "".to_string(),
        }
    }

    pub fn parse_items(&mut self, items: &[TextItem], transaction: &mut ProtoTransaction) -> usize {
        // Try reading and setting bounds from header
        let header_consumed = self.try_parse_header(items);
        if header_consumed > 0 {
            return header_consumed;
        }

        // Parser must be primed before parsing dates
        if !self.primed {
            return 0;
        }

        // Try parsing date
        let date_consumed = self.try_parse_date(items);
        if date_consumed > 0 {
            let date = self.date_parser.value.unwrap();
            transaction.date = Some(date);
            return date_consumed;
        }
        0
    }

    /// Set the starting year from current statement data
    pub fn set_start_date_year(&mut self, data: &StatementData) {
        self.start_date_year_str = if let Some(year) = data.start_date_year {
            year.to_string()
        } else {
            "".to_string()
        };
    }

    /// Reset the parser state
    pub fn reset(&mut self) {
        self.primed = false;
        self.date_parser.reset();
    }

    /// Set parser as primed
    pub fn prime(&mut self) {
        self.primed = true;
    }

    /// Get the maximum lookahead for the parser
    pub fn get_max_lookahead(&self) -> usize {
        let mut max_lookahead = 0;
        max_lookahead = max_lookahead.max(self.header_primer.max_lookahead);
        max_lookahead = max_lookahead.max(self.date_parser.max_lookahead);
        max_lookahead
    }

    /// Check if header is set
    pub fn is_header_set(&self) -> bool {
        self.header_primer.primed
    }

    /// Get effective x_bounds
    pub fn get_x_bounds(&self) -> (i32, i32) {
        let mut x_lower = 0;
        let mut x_upper = 10000;
        if self.alignment == "x1" {
            x_lower = self.x1_range[0];
            x_upper = self.x1_range[1];
        } else if self.alignment == "x2" {
            x_lower = self.x2_range[0];
            x_upper = self.x2_range[1];
        }
        (x_lower, x_upper)
    }

    /// Try reading header and set x_ranges accordingly
    fn try_parse_header(&mut self, items: &[TextItem]) -> usize {
        // Return if header already read
        if self.header_primer.primed {
            return 0;
        }
        let header_consumed = self.header_primer.parse_items(items);
        if header_consumed > 0 {
            let item = self.header_primer.text_item.as_ref().unwrap();
            if self.alignment == "x1" {
                self.x1_range = vec![item.x1 - self.x_tol, item.x1 + self.x_tol];
            } else if self.alignment == "x2" {
                self.x2_range = vec![item.x2 - self.x_tol, item.x2 + self.x_tol];
            }
        }
        header_consumed
    }

    /// Try parsing date and check if in x_ranges
    fn try_parse_date(&mut self, items: &[TextItem]) -> usize {
        let consumed = self
            .date_parser
            .parse_items(items, self.start_date_year_str.as_ref());
        if consumed == 0 {
            return 0;
        }
        // Check if date falls within x_ranges
        let item = self.date_parser.text_item.as_ref().unwrap();
        let x1_ok = item.x1 >= self.x1_range[0] && item.x1 <= self.x1_range[1];
        let x2_ok = item.x2 >= self.x2_range[0] && item.x2 <= self.x2_range[1];
        if !x1_ok || !x2_ok {
            // Reset date parser state
            self.date_parser.reset();
            return 0;
        }
        consumed
    }
}