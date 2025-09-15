use std::{env, fs, path::Path, process};
use transtractor::parsers;

fn print_usage(program: &str) {
    eprintln!("Usage: {program} <input.pdf> <output.txt>\n\nOutputs layout text representation of PDF.");
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
        // Extract text items and serialize layout text
        let items = parsers::text_items_from_pdf::extract_text_items(input);
        let layout = items.to_layout_text();
        if let Err(e) = fs::write(output, &layout.0) {
            eprintln!("Failed to write output file {output}: {e}");
            process::exit(1);
        }
        // Print the layout text to stdout as requested
        println!("{}", layout.0);
        return;
    }

    // Print usage and exit if arguments are missing or incorrect
    print_usage(&args[0]);
    process::exit(1);
}
