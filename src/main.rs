use std::{env, path::Path, process};
use transtractor::parsers::parser::Parser;

fn print_usage(program: &str) {
    eprintln!(
        "Usage:\n  {program} <input.pdf> <output.txt>  # PDF -> layout text (prints to stdout)\n  {program} <input.pdf> <output.csv>  # PDF -> CSV (prints StatementData to stdout for now)\n  {program} <input.txt> <output.csv>  # Layout text -> CSV (not implemented yet)\n"
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let parser = Parser::new();
    if args.len() == 3 {
        let input = &args[1];
        let output = &args[2];
        if !Path::new(input).exists() {
            eprintln!("Input file does not exist: {input}");
            process::exit(1);
        }
        let input_ext = Path::new(input)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();
        let output_ext = Path::new(output)
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();

        match (input_ext.as_str(), output_ext.as_str()) {
            // Mode 1: PDF -> layout text
            ("pdf", "txt") => {
                if let Err(e) = parser.to_layout_text(input, output, false) {
                    eprintln!("Failed to write layout text file {output}: {e}");
                    process::exit(1);
                }
                return;
            }
            // Mode 2: PDF -> CSV
            ("pdf", "csv") => {
                if let Err(e) = parser.to_csv(input, output) {
                    eprintln!("Failed to write CSV file {output}: {e}");
                    process::exit(1);
                }
                return;
            }
            // Mode 3: Layout text -> CSV
            ("txt", "csv") => {
                if let Err(e) = parser.to_csv(input, output) {
                    eprintln!("Failed to write CSV file {output}: {e}");
                    process::exit(1);
                }
                return;
            }
            _ => {
                // Fall through to usage
            }
        }
    }

    // Print usage and exit if arguments are missing or incorrect
    print_usage(&args[0]);
    process::exit(1);
}
