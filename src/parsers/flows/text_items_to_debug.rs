use crate::parsers::flows::text_items_to_statement_datas::text_items_to_statement_datas;
use crate::structs::TextItem;
use crate::structs::StatementConfig;


/// Parse non-tokenised text items into debug information string,
/// using provided statement configurations.
pub fn text_items_to_debug(
    items: &Vec<TextItem>,
    configs: &Vec<StatementConfig>,
) -> Result<String, String> {
    // Write debug information to the output file
    let mut output = String::new();
    output.push_str("Debug output\n");

    match text_items_to_statement_datas(&items, &configs) {
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
