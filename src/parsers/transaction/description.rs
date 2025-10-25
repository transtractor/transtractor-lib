use crate::parsers::base::ParserPrimer;
use crate::structs::{ProtoTransaction, StatementConfig, TextItem};

pub struct TransactionDescriptionParser {
    pub primed: bool,
    header_primer: ParserPrimer,
    alignment: String,
    x1_range: Vec<i32>,
    x2_range: Vec<i32>,
    x_tol: i32,
}

impl TransactionDescriptionParser {
    pub fn new(config: &StatementConfig) -> Self {
        let primer_terms: Vec<&str> = config
            .transaction_description_headers
            .iter()
            .map(|s| s.as_str())
            .collect();
        let alignment = config.transaction_description_alignment.clone();
        let x_tol = config.transaction_x_tol;
        Self {
            primed: false,
            header_primer: ParserPrimer::new(primer_terms.as_slice()),
            alignment,
            x_tol,
            x1_range: vec![0, 10000],
            x2_range: vec![0, 10000],
        }
    }

    pub fn parse_items(&mut self, items: &[TextItem], transaction: &mut ProtoTransaction) -> usize {
        // Try reading and setting bounds from header
        let header_consumed = self.try_parse_header(items);
        if header_consumed > 0 {
            return header_consumed;
        }

        // Parser must be primed before parsing descriptions
        if !self.primed {
            return 0;
        }

        // Try parsing description
        let description_consumed = self.try_parse_description(items);
        if description_consumed > 0 {
            // Append text of first item to description
            let mut description = transaction.description.clone();
            if !description.is_empty() {
                description.push(' ');
            }
            description.push_str(&items[0].text);
            transaction.description = description;
            return description_consumed;
        }
        0
    }

    /// Reset the parser state
    pub fn reset(&mut self) {
        self.primed = false;
    }

    /// Set parser as primed
    pub fn prime(&mut self) {
        self.primed = true;
    }

    /// Get the maximum lookahead for the parser
    pub fn get_max_lookahead(&self) -> usize {
        let mut max_lookahead = 0;
        max_lookahead = max_lookahead.max(self.header_primer.max_lookahead);
        max_lookahead
    }

    /// Try reading header and define x1 of x2 bounds
    fn try_parse_header(&mut self, items: &[TextItem]) -> usize {
        // Return if header already read
        if self.header_primer.primed {
            return 0;
        }
        let header_consumed = self.header_primer.parse_items(items);
        if header_consumed > 0 {
            let item = self.header_primer.text_item.as_ref().unwrap();
            if self.alignment == "x1" {
                self.x1_range = vec![item.x1 - self.x_tol, 10000];
            } else if self.alignment == "x2" {
                self.x2_range = vec![0, item.x2 + self.x_tol];
            }
        }
        header_consumed
    }

    /// Try parsing description - x1 and x2 of first item must be within ranges
    fn try_parse_description(&mut self, items: &[TextItem]) -> usize {
        if items.is_empty() {
            return 0;
        }
        let item = &items[0];
        if item.x1 >= self.x1_range[0]
            && item.x1 <= self.x1_range[1]
            && item.x2 >= self.x2_range[0]
            && item.x2 <= self.x2_range[1]
        {
            return 1; // Consumed 1 item
        }
        0
    }
}