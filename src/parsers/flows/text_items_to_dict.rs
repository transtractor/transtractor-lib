use crate::configs::StatementTyper;
use crate::parsers::flows::statement_data_to_dict::ColumnData;
use crate::parsers::flows::statement_data_to_dict::statement_data_to_dict;
use crate::parsers::flows::text_items_to_statement_datas::text_items_to_statement_datas;
use crate::structs::TextItems;
use std::collections::HashMap;

/// Parses TextItems to to a dictionary of lists for reading into a DataFrame.
///
/// For PDF files, extracts text items using PDF parsing.
/// For TXT files, reads layout text format and parses into text items.
///
/// Returns a dictionary with column names as keys and vectors of typed data as values.
/// Returns an error if no StatementData is error-free.
pub fn text_items_to_dict(
    items: &mut TextItems,
    typer: &StatementTyper,
) -> Result<HashMap<String, ColumnData>, String> {
    let statement_data_results = text_items_to_statement_datas(items, typer)?;

    // Find the first error-free StatementData
    for data in statement_data_results {
        if data.errors.is_empty() {
            // Convert the first error-free result to dictionary
            return Ok(statement_data_to_dict(&data));
        }
    }
    Err("Extracted data failed quality check indicating an issue with statement parsing configuration.".to_string())
}