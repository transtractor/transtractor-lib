use std::{env, fs, path::Path, process};
use transtractor::configs::StatementTyper;
use transtractor::parsers;

fn print_usage(program: &str) {
    eprintln!(
        "Usage:\n  {program} <input.pdf> <output.txt>  # PDF -> layout text (prints to stdout)\n  {program} <input.pdf> <output.csv>  # PDF -> CSV (prints StatementData to stdout for now)\n  {program} <input.txt> <output.csv>  # Layout text -> CSV (not implemented yet)\n"
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
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
                let items = parsers::text_items_from_pdf::parse(input);
                let layout = items.to_layout_text();
                if let Err(e) = fs::write(output, &layout.0) {
                    eprintln!("Failed to write output file {output}: {e}");
                    process::exit(1);
                }
                // Also print the layout text to stdout
                println!("{}", layout.0);
                return;
            }
            // Mode 2: PDF -> CSV (for now, build StatementData and print it; file is a placeholder)
            ("pdf", "csv") => {
                let items = parsers::text_items_from_pdf::parse(input);
                let typer = StatementTyper::new();
                match typer.identify_from_text_items(&items) {
                    Some(mut cfgs) if !cfgs.is_empty() => {
                        // Choose the first config for now
                        let cfg = cfgs.remove(0);
                        // Apply y-disorder fix before parsing if apply_y_patch true
                        let items = if cfg.apply_y_patch {
                            items.fix_y_disorder()
                        } else {
                            items
                        };
                        let data = transtractor::parsers::statement_data_from_text_items::parse(&cfg, &items);
                        // Print StatementData to stdout (temporary behavior)
                        data.print();
                        println!("Layout Text:");
                        items.print_layout();
                        // Write a placeholder to the CSV file to acknowledge the request
                        let note = format!(
                            "CSV generation not implemented yet. Parsed with config '{}'. See stdout for StatementData debug output.",
                            cfg.key
                        );
                        if let Err(e) = fs::write(output, note.as_bytes()) {
                            eprintln!("Failed to write output file {output}: {e}");
                            process::exit(1);
                        }
                        return;
                    }
                    _ => {
                        eprintln!("Could not identify statement type from PDF; no matching configuration found.");
                        process::exit(2);
                    }
                }
            }
            // Mode 3: Layout text -> CSV (placeholder)
            ("txt", "csv") => {
                eprintln!("Layout text -> CSV mode is not implemented yet.");
                process::exit(3);
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
