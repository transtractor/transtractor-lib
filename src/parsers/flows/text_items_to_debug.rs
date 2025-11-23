use crate::configs::StatementTyper;
use crate::parsers::flows::text_items_to_statement_datas::text_items_to_statement_datas;
use crate::structs::TextItems;

/// Read TextItems and record all parsed StatementData results into string for debugging.
/// This provides detailed debug information about all parsing attempts and their results.
/// The string can then be written to a file or logged as needed.
pub fn text_items_to_debug(items: &mut TextItems, typer: &StatementTyper) -> Result<String, String> {
    // Write debug information to the output file
    let mut output = String::new();
    output.push_str("Debug output\n");

    match text_items_to_statement_datas(items, typer) {
        Ok(statement_data_results) => {
            output.push_str(&format!(
                "Found {} StatementData result(s)\n\n",
                statement_data_results.len()
            ));

            for (i, data) in statement_data_results.iter().enumerate() {
                output.push_str(&format!("=== StatementData Result {} ===\n", i + 1));
                output.push_str(&data.to_string());
                output.push_str("\n");
            }
        }
        Err(error) => {
            output.push_str("Error: Failed to identify statement type or parse text items\n");
            output.push_str(&format!("Error details: {}\n\n", error));
        }
    }
    Ok(output)
}
