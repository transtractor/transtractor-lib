# Architecture Guide
The Transtractor is implemented in **Python** and **Rust**. Rust is used not primarily for performance, but to ensure the core processing engine can be ported across multiple languages and runtimes. Python bindings make it easy to integrate Transtractor into backend systems for data extraction and analysis. WASM bindings will also be developed to support static web applications such as [Transtractor.net](https://www.transtractor.net/), for secure, in‑browser parsing of bank statements.

## Parsing Pipeline
Transtractor first extracts PDF text into **tokenised word units** using a high‑level language like Python. These tokens are then passed to the **Rust processing engine**, which uses key terms, token order, and token coordinates to reconstruct transaction records and metadata such as account numbers and opening/closing balances. The Rust engine performs extensive validation before returning structured results to the high‑level language for further analysis.

### Step 1: Extract PDF Content
PDF text extraction is performed using the `pdfplumber` Python package, which reliably handles many fonts, whitespace, and bounding‑box coordinates. At present, no Rust‑based equivalent offers comparable accuracy. If one emerges, replacing this component would help reduce variability introduced by different PDF parsers.

All Python‑side parsing is encapsulated in the `Parser` class. When instantiated, it automatically loads configuration data from the JSON files stored as JSON under `python/transtractor/configs/` and its subdirectories. A single `Parser` instance can process multiple statements, so configuration files only need to be loaded once. 

The first stage extracts all PDF text into **single‑word tokens**, split by whitespace. The `pdf_to_text_items` function acts as the **PDF parsing interface**, producing token structures compatible with `py_text_items_to_rust_text_items`, which serves as the **Python-to-Rust input interface**.

Once tokens are extracted, the `Parser` makes two calls to the Rust backend:

1. **Statement classification** via the Rust `StatementTyper`, which determines the statement type using a keyword-based algorithm.
2. **Transaction extraction**, where the Rust engine processes tokens into structured transaction data.

This two‑step approach ensures the `Parser` only loads the minimal configuration upfront (specifically the `account terms` field). Additional configuration is cached only after the statement type is known. This prevents the `Parser` from becoming excessively large if the Transtractor scales to hundreds or thousands of supported statement formats.

### Step 2: Process tokens into Structured Transaction Data
The `Parser`’s second call instructs the Rust engine to process the extracted tokens according to one or more candidate statement types returned by the `StatementTyper`.

The top‑level Rust entry point, `text_items_to_statement_datas`, iterates through all relevant configurations and invokes `text_items_to_statement_data` for each one. This continues until a configuration produces a valid, error‑free `StatementData` object containing fully structured and internally validated results.

This iterative behaviour is essential because statement formats evolve over time, and multiple versions may share identical keyword signatures. By attempting each configuration in turn, the engine can reliably select the correct format even when classification alone is ambiguous.

Once a valid `StatementData` is produced, results are returned to Python through `rust_statement_data_to_py_statement_data`, the **Rust‑to‑Python output interface**. All subsequent processing and data handling occur on the Python side using the `StatementData` transfer class.

The Transtractor’s core parsing model separates **reusable logic** from **format‑specific rules**. All general parsing behaviour is implemented directly in the Rust engine, while statement‑specific formatting details are defined in lightweight JSON configuration files. This avoids building a bespoke parser for every individual statement format. Even though statements can look very different, they share structural patterns that the Transtractor can exploit and eventually evolve into a genuinely universal parser.

### Step 3: Using the Data
The Python‑side `StatementData` transfer class provides methods for exporting results either as CSV files or as Pandas‑compatible dictionaries, making the data easy to load into spreadsheet tools or integrate into broader analytical workflows. This is where the scope of this repository ends as the downstream use of the extracted data is intentionally open‑ended and entirely up to the user.

For me, this parsing library is a foundational component of the [Transtractor.net](https://www.transtractor.net/) personal budgeting app I build in my spare time. Other users may choose to incorporate it into their own personal finance tools or into business workflows that require routine, reliable extraction of financial data from bank statements.
