use crate::parsers;
use crate::configs::StatementTyper;
use crate::fixers::fix_statement_data;

pub struct Parser {
    typer: StatementTyper,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            typer: StatementTyper::new(),
        }
    }

    /// Converts a PDF bank statement to a CSV file.
    pub fn pdf_to_csv(&self, input_pdf: &str, output_csv: &str) -> Result<(), String> {
        // Check if file exists first
        if !std::path::Path::new(input_pdf).exists() {
            return Err(format!("Input PDF file does not exist: {}", input_pdf));
        }
        
        let items = parsers::text_items_from_pdf::parse(input_pdf);
        match self.typer.identify_from_text_items(&items) {
            Some(mut cfgs) if !cfgs.is_empty() => {
                let cfg = cfgs.remove(0);
                let items = if cfg.apply_y_patch {
                    items.fix_y_disorder()
                } else {
                    items
                };
                let mut data = parsers::statement_data_from_text_items::parse(&cfg, &items);
                
                // Apply fixers to clean up the data
                fix_statement_data(&mut data);
                
                // Write to CSV
                parsers::csv_from_statement_data::parse(&data, output_csv)
                    .map_err(|e| format!("Failed to write CSV: {}", e))?;
                
                Ok(())
            }
            _ => Err("Could not identify statement type from PDF; no matching configuration found.".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_new() {
        let _parser = Parser::new();
        // Just verify it creates without panicking
        assert!(true);
    }

    #[test]
    fn test_parser_pdf_to_csv_with_nonexistent_file() {
        let parser = Parser::new();
        let result = parser.pdf_to_csv("nonexistent.pdf", "output.csv");
        
        // Should return an error for a nonexistent file
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("does not exist"));
    }
}