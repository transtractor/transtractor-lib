use crate::parsers::statement::{ClosingBalanceParser, OpeningBalanceParser};
use crate::structs::StatementConfig;
use crate::structs::StatementData;
use crate::structs::TextItems;

pub fn parse(config: &StatementConfig, text_items: &TextItems) -> StatementData {
    let mut statement_data = StatementData::new();

    // Initialize parsers
    let mut opening_balance_parser = OpeningBalanceParser::new(config);
    let mut closing_balance_parser = ClosingBalanceParser::new(config);

    // Other settings based on parsers
    let max_lookahead = opening_balance_parser
        .get_max_lookahead()
        .max(closing_balance_parser.get_max_lookahead());

    // Iterate through text items, attempting to match account_terms
    let len = text_items.len();
    if len == 0 {
        return statement_data;
    }
    let mut i: usize = 0;
    while i < len {
        let buffer_size = max_lookahead.min(len - i);
        let buffer = text_items.get_text_item_buffer(i, buffer_size);
        let mut consumed = 0usize;
        // Try opening balance first
        let consumed_open = opening_balance_parser.parse_items(&buffer, &mut statement_data);
        if consumed_open > 0 {
            consumed = consumed_open;
        } else {
            // Try closing balance if opening didn't consume
            let consumed_close = closing_balance_parser.parse_items(&buffer, &mut statement_data);
            if consumed_close > 0 {
                consumed = consumed_close;
            }
        }

        if consumed > 0 {
            i += consumed;
            continue;
        }

        // No parser matched, move to next item
        i += 1;
    }
    statement_data
}