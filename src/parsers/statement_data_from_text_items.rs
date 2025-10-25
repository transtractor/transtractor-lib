use crate::parsers::statement::{
    ClosingBalanceParser, OpeningBalanceParser, StartDateParser, TransactionParser,
};
use crate::structs::StatementConfig;
use crate::structs::StatementData;
use crate::structs::TextItems;

pub fn parse(config: &StatementConfig, text_items: &TextItems) -> StatementData {
    let mut statement_data = StatementData::new();

    // Initialize parsers
    let mut opening_balance_parser = OpeningBalanceParser::new(config);
    let mut closing_balance_parser = ClosingBalanceParser::new(config);
    let mut start_date_parser = StartDateParser::new(config);
    let mut transaction_parser = TransactionParser::new(config);

    // Other settings based on parsers
    // Compute max lookahead across all parsers generically to keep this scalable
    let lookaheads = [
        opening_balance_parser.get_max_lookahead(),
        closing_balance_parser.get_max_lookahead(),
        start_date_parser.get_max_lookahead(),
        transaction_parser.get_max_lookahead(),
    ];
    let max_lookahead = *lookaheads.iter().max().unwrap_or(&0);

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
        // Try parsers in a stable order: start date -> opening balance -> closing balance
        if consumed == 0 {
            consumed = start_date_parser.parse_items(&buffer, &mut statement_data);
        }
        if consumed == 0 {
            consumed = opening_balance_parser.parse_items(&buffer, &mut statement_data);
        }
        if consumed == 0 {
            consumed = closing_balance_parser.parse_items(&buffer, &mut statement_data);
        }
        if consumed == 0 {
            consumed = transaction_parser.parse_items(&buffer, &mut statement_data);
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
