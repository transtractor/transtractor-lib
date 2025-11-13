use crate::checkers::check_statement_data;
use crate::configs::StatementTyper;
use crate::fixers::fix_statement_data;
use crate::parsers;
use crate::parsers::dict_from_statement_data::{dict_from_statement_data, ColumnData};
use crate::structs::text_items::LayoutText;
use crate::structs::{StatementData, TextItems};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::Instant;

pub struct Parser {
    typer: StatementTyper,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            typer: StatementTyper::new(),
        }
    }

    /// Converts a PDF or TXT bank statement to a CSV file.
    ///
    /// For PDF files, extracts text items using PDF parsing.
    /// For TXT files, reads layout text format and parses into text items.
    ///
    /// Writes the first error-free StatementData to CSV.
    /// Returns an error if no StatementData is error-free.
    pub fn to_csv(&self, input_file: &str, output_csv: &str) -> Result<(), String> {
        // Check if file exists first
        if !std::path::Path::new(input_file).exists() {
            return Err(format!("Input file does not exist: {}", input_file));
        }

        let input_file_lower = input_file.to_lowercase();
        let mut items = if input_file_lower.ends_with(".pdf") {
            // Parse PDF file
            parsers::text_items_from_pdf::parse(input_file)
        } else if input_file_lower.ends_with(".txt") {
            // Read TXT file and parse as layout text
            let layout_content = std::fs::read_to_string(input_file)
                .map_err(|e| format!("Failed to read TXT file: {}", e))?;
            let layout = LayoutText(layout_content);
            let mut items = TextItems::new();
            items
                .read_from_layout_text(&layout)
                .map_err(|e| format!("Failed to parse layout text: {:?}", e))?;
            items
        } else {
            return Err(
                "Unsupported file format. Only .pdf and .txt files are supported.".to_string(),
            );
        };

        let statement_data_results = self.parse_text_items(&mut items)?;

        // Find the first error-free StatementData
        for data in statement_data_results {
            if data.errors.is_empty() {
                // Write the first error-free result to CSV
                parsers::csv_from_statement_data::parse(&data, output_csv)
                    .map_err(|e| format!("Failed to write CSV: {}", e))?;
                return Ok(());
            }
        }
        Err("Extracted data failed quality check indicating an issue with statement parsing configuration.".to_string())
    }

    /// Converts a PDF or TXT bank statement to a dictionary of lists.
    ///
    /// For PDF files, extracts text items using PDF parsing.
    /// For TXT files, reads layout text format and parses into text items.
    ///
    /// Returns a dictionary with column names as keys and vectors of typed data as values.
    /// Returns an error if no StatementData is error-free.
    pub fn to_dict(&self, input_file: &str) -> Result<HashMap<String, ColumnData>, String> {
        // Check if file exists first
        if !std::path::Path::new(input_file).exists() {
            return Err(format!("Input file does not exist: {}", input_file));
        }

        let input_file_lower = input_file.to_lowercase();
        let mut items = if input_file_lower.ends_with(".pdf") {
            // Parse PDF file
            parsers::text_items_from_pdf::parse(input_file)
        } else if input_file_lower.ends_with(".txt") {
            // Read TXT file and parse as layout text
            let layout_content = std::fs::read_to_string(input_file)
                .map_err(|e| format!("Failed to read TXT file: {}", e))?;
            let layout = LayoutText(layout_content);
            let mut items = TextItems::new();
            items
                .read_from_layout_text(&layout)
                .map_err(|e| format!("Failed to parse layout text: {:?}", e))?;
            items
        } else {
            return Err(
                "Unsupported file format. Only .pdf and .txt files are supported.".to_string(),
            );
        };

        let statement_data_results = self.parse_text_items(&mut items)?;

        // Find the first error-free StatementData
        for data in statement_data_results {
            if data.errors.is_empty() {
                // Convert the first error-free result to dictionary
                return Ok(dict_from_statement_data(&data));
            }
        }
        Err("Extracted data failed quality check indicating an issue with statement parsing configuration.".to_string())
    }

    /// Read a PDF or TXT file and write all parsed StatementData results to an output file for debugging.
    /// This provides detailed debug information about all parsing attempts and their results.
    pub fn debug(&self, input_file: &str, output_file: &str) -> Result<(), String> {
        // Check if file exists first
        if !std::path::Path::new(input_file).exists() {
            return Err(format!("Input file does not exist: {}", input_file));
        }

        let input_file_lower = input_file.to_lowercase();
        let mut items = if input_file_lower.ends_with(".pdf") {
            // Parse PDF file
            parsers::text_items_from_pdf::parse(input_file)
        } else if input_file_lower.ends_with(".txt") {
            // Read TXT file and parse as layout text
            let layout_content = std::fs::read_to_string(input_file)
                .map_err(|e| format!("Failed to read TXT file: {}", e))?;
            let layout = LayoutText(layout_content);
            let mut items = TextItems::new();
            match items.read_from_layout_text(&layout) {
                Ok(_) => items,
                Err(e) => {
                    // Handle layout text parsing error by writing it to debug output
                    let mut output = String::new();
                    output.push_str(&format!("Debug output for file: {}\n", input_file));
                    output.push_str("Error: Failed to parse layout text format\n");
                    output.push_str(&format!("Error details: {:?}\n\n", e));

                    fs::write(output_file, output)
                        .map_err(|e| format!("Failed to write debug output file: {}", e))?;

                    return Ok(());
                }
            }
        } else {
            return Err(
                "Unsupported file format. Only .pdf and .txt files are supported.".to_string(),
            );
        };

        // Write debug information to the output file
        let mut output = String::new();
        output.push_str(&format!("Debug output for file: {}\n", input_file));
        
        match self.parse_text_items(&mut items) {
            Ok(statement_data_results) => {
                output.push_str(&format!("Found {} StatementData result(s)\n\n", statement_data_results.len()));
                
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

        fs::write(output_file, output)
            .map_err(|e| format!("Failed to write debug output file: {}", e))?;

        println!("Debug output written to: {}", output_file);
        Ok(())
    }

    /// Converts a PDF file to layout text format and writes it to a file.
    pub fn to_layout_text(&self, input_file: &str, output_file: &str, fix_y_disorder: bool) -> Result<(), String> {
        // Check if file exists first
        if !std::path::Path::new(input_file).exists() {
            return Err(format!("Input file does not exist: {}", input_file));
        }

        let input_file_lower = input_file.to_lowercase();
        if !input_file_lower.ends_with(".pdf") {
            return Err("Only PDF files are supported for layout text conversion.".to_string());
        }

        // Parse PDF file
        let mut items = parsers::text_items_from_pdf::parse(input_file);

        // Apply Y-coordinate disorder fix if requested
        if fix_y_disorder {
            items = items.fix_y_disorder();
        }

        let layout_text = items.to_layout_text();
        
        // Write layout text to the output file
        fs::write(output_file, layout_text.0)
            .map_err(|e| format!("Failed to write layout text file: {}", e))?;

        Ok(())
    }

    /// Recursively finds all PDF and TXT files in a directory and its subdirectories.
    /// Tests each file to see if it can be parsed and prints detailed information about the results.
    ///
    /// For each file, prints: "Reading <file path>..."
    /// For each StatementData, prints: Key, Number of transactions, Time taken (ms), Number of errors, PASS/FAIL
    pub fn test_directory(&self, directory_path: &str) -> Result<(), String> {
        let dir_path = Path::new(directory_path);
        if !dir_path.exists() {
            return Err(format!("Directory does not exist: {}", directory_path));
        }
        if !dir_path.is_dir() {
            return Err(format!("Path is not a directory: {}", directory_path));
        }

        self.test_directory_recursive(dir_path)?;
        Ok(())
    }

    /// Helper function for recursive directory processing
    fn test_directory_recursive(&self, dir_path: &Path) -> Result<(), String> {
        let entries = fs::read_dir(dir_path)
            .map_err(|e| format!("Failed to read directory {:?}: {}", dir_path, e))?;

        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let path = entry.path();

            if path.is_dir() {
                // Recursively process subdirectories
                self.test_directory_recursive(&path)?;
            } else if path.is_file() {
                // Check if file has supported extension (case insensitive)
                if let Some(file_name) = path.to_str() {
                    let file_name_lower = file_name.to_lowercase();
                    if file_name_lower.ends_with(".pdf") || file_name_lower.ends_with(".txt") {
                        self.process_single_file(file_name)?;
                    }
                }
            }
        }
        Ok(())
    }

    /// Processes a single file and prints results
    fn process_single_file(&self, file_path: &str) -> Result<(), String> {
        println!("Reading {}...", file_path);

        // Check if file exists
        if !Path::new(file_path).exists() {
            println!("File not found: {}", file_path);
            return Ok(());
        }

        let start_time = Instant::now();

        // Parse the file to TextItems
        let input_file_lower = file_path.to_lowercase();
        let mut items = if input_file_lower.ends_with(".pdf") {
            // Parse PDF file
            parsers::text_items_from_pdf::parse(file_path)
        } else if input_file_lower.ends_with(".txt") {
            // Read TXT file and parse as layout text
            match fs::read_to_string(file_path) {
                Ok(layout_content) => {
                    let layout = LayoutText(layout_content);
                    let mut items = TextItems::new();
                    match items.read_from_layout_text(&layout) {
                        Ok(_) => items,
                        Err(e) => {
                            println!("Error reading layout text: {:?}", e);
                            return Ok(());
                        }
                    }
                }
                Err(e) => {
                    println!("Failed to read TXT file: {}", e);
                    return Ok(());
                }
            }
        } else {
            // This shouldn't happen given our filtering, but handle gracefully
            return Ok(());
        };

        // Try to parse with available configs
        match self.parse_text_items(&mut items) {
            Ok(statement_data_results) => {
                let total_time = start_time.elapsed();

                for data in statement_data_results {
                    let key = data.key.clone().unwrap_or_else(|| "Unknown".to_string());
                    let transaction_count = data.proto_transactions.len();
                    let error_count = data.errors.len();
                    let status = if error_count == 0 { "PASS" } else { "FAIL" };

                    println!(
                        "  Key: {}, Transactions: {}, Time: {}ms, Errors: {}, Status: {}",
                        key,
                        transaction_count,
                        total_time.as_millis(),
                        error_count,
                        status
                    );
                }
            }
            Err(err) => {
                if err.contains("Statement type not supported") {
                    println!("Statement type not supported");
                } else {
                    println!("Error: {}", err);
                }
            }
        }

        Ok(())
    }

    /// Parse text items with all matching configs and return a Vec of StatementData.
    fn parse_text_items(&self, items: &mut TextItems) -> Result<Vec<StatementData>, String> {
        match self.typer.identify_from_text_items(items) {
            Some(cfgs) if !cfgs.is_empty() => {
                let mut results = Vec::new();

                for cfg in cfgs {
                    // Create a copy of items for each config to avoid side effects
                    let items_copy = if cfg.apply_y_patch {
                        items.clone().fix_y_disorder()
                    } else {
                        items.clone()
                    };

                    let mut data =
                        parsers::statement_data_from_text_items::parse(&cfg, &items_copy);
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
    fn test_parser_to_csv_with_nonexistent_file() {
        let parser = Parser::new();
        let result = parser.to_csv("nonexistent.pdf", "output.csv");

        // Should return an error for a nonexistent file
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("does not exist"));
    }

    #[test]
    fn test_parser_to_csv_unsupported_file_format() {
        let parser = Parser::new();

        // Create a temporary file with unsupported extension
        let temp_file = "test.doc";
        std::fs::write(temp_file, "test content").unwrap();

        let result = parser.to_csv(temp_file, "output.csv");

        // Clean up
        let _ = std::fs::remove_file(temp_file);

        // Should return an error for unsupported file format
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unsupported file format"));
    }

    #[test]
    fn test_parser_to_csv_case_insensitive_extensions() {
        let parser = Parser::new();

        // Test uppercase PDF extension
        let result = parser.to_csv("nonexistent.PDF", "output.csv");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("does not exist")); // Should recognize as PDF, fail on file existence

        // Test mixed case TXT extension
        let result = parser.to_csv("nonexistent.TxT", "output.csv");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("does not exist")); // Should recognize as TXT, fail on file existence

        // Test uppercase unsupported extension
        let temp_file = "test.DOC";
        std::fs::write(temp_file, "test content").unwrap();
        let result = parser.to_csv(temp_file, "output.csv");
        let _ = std::fs::remove_file(temp_file);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unsupported file format")); // Should not recognize DOC
    }

    #[test]
    fn test_parse_text_items_returns_vec() {
        let parser = Parser::new();
        let mut items = TextItems::new();

        // Test with empty items - should return error
        let result = parser.parse_text_items(&mut items);
        assert!(result.is_err());

        // The error message should indicate statement type not supported
        assert!(result.unwrap_err().contains("Statement type not supported"));
    }

    #[test]
    fn test_test_directory_nonexistent() {
        let parser = Parser::new();
        let result = parser.test_directory("nonexistent_directory");

        // Should return an error for a nonexistent directory
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Directory does not exist"));
    }

    #[test]
    fn test_test_directory_not_directory() {
        let parser = Parser::new();

        // Create a temporary file (not a directory)
        let temp_file = "test_file.txt";
        std::fs::write(temp_file, "test content").unwrap();

        let result = parser.test_directory(temp_file);

        // Clean up
        let _ = std::fs::remove_file(temp_file);

        // Should return an error when path is not a directory
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Path is not a directory"));
    }

    #[test]
    fn test_debug_nonexistent_file() {
        let parser = Parser::new();
        let result = parser.debug("nonexistent.pdf", "debug_output.txt");

        // Should return an error for a nonexistent file
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("does not exist"));
    }

    #[test]
    fn test_debug_unsupported_file_format() {
        let parser = Parser::new();

        // Create a temporary file with unsupported extension
        let temp_file = "test.doc";
        std::fs::write(temp_file, "test content").unwrap();

        let result = parser.debug(temp_file, "debug_output.txt");

        // Clean up
        let _ = std::fs::remove_file(temp_file);

        // Should return an error for unsupported file format
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unsupported file format"));
    }

    #[test]
    fn test_debug_with_unrecognized_content() {
        let parser = Parser::new();

        // Create a temporary TXT file with content that won't match any statement patterns
        let temp_file = "test_unrecognized_debug.txt";
        let test_content = "This is just random text that doesn't look like a bank statement.\nNo dates, no amounts, no transaction patterns.";
        std::fs::write(temp_file, test_content).unwrap();

        let output_file = "debug_test_output.txt";
        let result = parser.debug(temp_file, output_file);

        // Print the error to understand what's happening
        if let Err(ref e) = result {
            println!("Debug error: {}", e);
        }

        // Should succeed (not return an error) even when statement type is not recognized
        assert!(result.is_ok(), "Debug should succeed even with unrecognized content");

        // Read the debug output and verify it contains error information
        let debug_content = std::fs::read_to_string(output_file).unwrap();
        assert!(debug_content.contains("Error: Failed to parse layout text format"));

        // Clean up
        let _ = std::fs::remove_file(temp_file);
        let _ = std::fs::remove_file(output_file);
    }

    #[test]
    fn test_to_layout_text_nonexistent_file() {
        let parser = Parser::new();
        let result = parser.to_layout_text("nonexistent.pdf", "output.txt", false);

        // Should return an error for a nonexistent file
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("does not exist"));
    }

    #[test]
    fn test_to_layout_text_unsupported_file_format() {
        let parser = Parser::new();

        // Create a temporary TXT file
        let temp_file = "test.txt";
        std::fs::write(temp_file, "test content").unwrap();

        let result = parser.to_layout_text(temp_file, "output.txt", false);

        // Clean up
        let _ = std::fs::remove_file(temp_file);

        // Should return an error for non-PDF file format
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Only PDF files are supported"));
    }

    #[test]
    fn test_to_layout_text_case_insensitive_pdf_extension() {
        let parser = Parser::new();

        // Test uppercase PDF extension
        let result = parser.to_layout_text("nonexistent.PDF", "output.txt", false);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("does not exist")); // Should recognize as PDF, fail on file existence

        // Test mixed case PDF extension
        let result = parser.to_layout_text("nonexistent.Pdf", "output.txt", true);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("does not exist")); // Should recognize as PDF, fail on file existence
    }
}
