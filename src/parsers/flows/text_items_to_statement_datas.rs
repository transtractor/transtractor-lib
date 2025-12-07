use crate::checkers::check_statement_data;
use crate::configs::StatementTyper;
use crate::fixers::fix_statement_data;
use crate::parsers::flows::text_items_to_statement_data::text_items_to_statement_data;
use crate::structs::StatementData;
use crate::structs::TextItem;
use crate::structs::text_items::sort_items;
use crate::structs::text_items::tokenise_items;

/// Parses text items into statement data based on identified statement configurations.
/// Returns a vector of StatementData or an error message if the statement type is not supported.
pub fn text_items_to_statement_datas(
    items: &Vec<TextItem>,
    typer: &StatementTyper,
) -> Result<Vec<StatementData>, String> {
    let tokenised_items = tokenise_items(items.clone());
    match typer.identify_from_text_items(&tokenised_items) {
        Some(cfgs) if !cfgs.is_empty() => {
            let mut results = Vec::new();

            for cfg in cfgs {
                // Sort will just return a clone if y_bin is 0.0
                let sorted_items = sort_items(items, cfg.fix_text_order[1], cfg.fix_text_order[0]);
                let tokenised_sorted_items = tokenise_items(sorted_items);

                let mut data = text_items_to_statement_data(&cfg, &tokenised_sorted_items);
                data.set_key(cfg.key);

                // Apply fixers to clean up the data
                fix_statement_data(&mut data);
                check_statement_data(&mut data);

                results.push(data);
            }

            Ok(results)
        }
        _ => Err("Statement type not supported.".to_string()),
    }
}
