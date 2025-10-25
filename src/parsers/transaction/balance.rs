use crate::parsers::base::{AmountParser, ParserPrimer};
use crate::structs::{ProtoTransaction, StatementConfig, TextItem};

pub struct TransactionBalanceParser {
    pub primed: bool,
    balance_parser: AmountParser,
    header_primer: ParserPrimer,
    alignment: String,
    x1_range: Vec<i32>,
    x2_range: Vec<i32>,
    x_tol: i32,
    invert: bool,
}

impl TransactionBalanceParser {
    pub fn new(config: &StatementConfig) -> Self {
        let primer_terms: Vec<&str> = config
            .transaction_balance_headers
            .iter()
            .map(|s| s.as_str())
            .collect();
        let amount_formats: Vec<&str> = config
            .transaction_amount_formats
            .iter()
            .map(|s| s.as_str())
            .collect();
        let alignment = config.transaction_amount_alignment.clone();
        let x_tol = config.transaction_x_tol;
        let invert = config.transaction_amount_invert;
        Self {
            primed: false,
            balance_parser: AmountParser::new(amount_formats.as_slice()),
            header_primer: ParserPrimer::new(primer_terms.as_slice()),
            alignment,
            x_tol,
            x1_range: vec![0, 10000],
            x2_range: vec![0, 10000],
            invert,
        }
    }

    pub fn parse_items(&mut self, items: &[TextItem], transaction: &mut ProtoTransaction) -> usize {
        // Try reading and setting bounds from header
        let header_consumed = self.try_parse_header(items);
        if header_consumed > 0 {
            return header_consumed;
        }

        // Parser must be primed before parsing balances
        if !self.primed {
            return 0;
        }

        // Try parsing balance
        let balance_consumed = self.try_parse_balance(items);
        if balance_consumed > 0 {
            let mut value = self.balance_parser.value.unwrap();
            if self.invert {
                value = -value;
            }
            transaction.balance = Some(value);
            return balance_consumed;
        }
        0
    }

    /// Reset the parser state
    pub fn reset(&mut self) {
        self.primed = false;
        self.balance_parser.reset();
    }

    /// Set parser as primed
    pub fn prime(&mut self) {
        self.primed = true;
    }

    /// Get the maximum lookahead for the parser
    pub fn get_max_lookahead(&self) -> usize {
        let mut max_lookahead = 0;
        max_lookahead = max_lookahead.max(self.header_primer.max_lookahead);
        max_lookahead = max_lookahead.max(self.balance_parser.max_lookahead);
        max_lookahead
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

    /// Try parsing balance and check if in x_ranges
    fn try_parse_balance(&mut self, items: &[TextItem]) -> usize {
        let consumed = self.balance_parser.parse_items(items);
        if consumed == 0 {
            return 0; // No balance found
        }
        let item = self.balance_parser.text_item();
        // Check x1 and x2 ranges
        let x1_ok = item.x1 >= self.x1_range[0] && item.x1 <= self.x1_range[1];
        let x2_ok = item.x2 >= self.x2_range[0] && item.x2 <= self.x2_range[1];
        if !x1_ok || !x2_ok {
            // Reset balance parser state
            self.balance_parser.reset();
            return 0;
        }
        consumed
    }
}