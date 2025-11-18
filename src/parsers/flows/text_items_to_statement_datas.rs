use crate::checkers::check_statement_data;
use crate::configs::StatementTyper;
use crate::fixers::fix_statement_data;
use crate::parsers::flows::text_items_to_statement_data::text_items_to_statement_data;
use crate::structs::StatementData;
use crate::structs::TextItems;

/// Parses text items into statement data based on identified statement configurations.
/// Returns a vector of StatementData or an error message if the statement type is not supported.
pub fn text_items_to_statement_datas(
    items: &mut TextItems,
    typer: &StatementTyper,
) -> Result<Vec<StatementData>, String> {
    match typer.identify_from_text_items(items) {
        Some(cfgs) if !cfgs.is_empty() => {
            let mut results = Vec::new();

            for cfg in cfgs {
                // Create a copy of items for each config to avoid side effects
                let items_copy = if cfg.apply_y_patch {
                    items.clone().fix_y_disorder()
                } else {
                    items.clone()
                };

                let mut data = text_items_to_statement_data(&cfg, &items_copy);
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
