# The Transtractor (pre-release)

## Universal PDF bank statement parsing

The Transaction Extractor, or 'Transtractor', aspires to be a universal 
library for extracting transaction data from PDF bank statements. Key features:

* Written in Rust (fast)
* Python API (user friendly)
* AI-free (lightweight)
* Rules-based extraction (100% predictable and accurate)


## Installation

Transtractor is currently under active development and has not yet been released on PyPI. For now, you'll need to compile from source.

### Compile from source

1. **Install Rust**: Download and install Rust from [rustup.rs](https://rustup.rs/)

2. **Install Maturin**: Install the Python build tool for Rust extensions
   ```bash
   pip install maturin
   ```

3. **Build and install Transtractor**: Clone the repository and build
   ```bash
   git clone https://github.com/gravytoast/transtractor.git
   cd transtractor
   maturin develop --release
   ```

### Basic usage (Python)

1. **Import and initialise the parser**
   ```python
   from transtractor import Parser

   parser = Parser()
   ```

2. **Convert PDF to CSV**: All CSV files are written in a standard format
   ```python
   parser.parse('statement.pdf').to_csv('statement.csv')
   ```

3. **Convert PDF the dictionary**: Load into a DataFrame for analysis
   ```python
   import pandas as pd

   data = parser.parse('statement.pdf').to_pandas_dict()
   df = pd.DataFrame(data)
   ```

## Supported statements

Only a limited number of PDF statements are supported. This should expand with community contributions.

Currently supported statements, and their config files (src/configs), include:

### Australia

* **Commonwealth Bank**
    * Credit Card (*au__cba__credit_card__1*)
    * Debit/Savings (*au__cba__debit__1*)
    * Loan (*au__cba__loan__1*)

* **National Australia Bank**
    * Classic Banking (*au__nab__classic_banking__1*)

## Expanding support

Stay tuned for proper guidance. But if you are really keen:

1. Develop a config file for your bank statement and copy it into the src/configs folder. 
2. Add new date or amount formats as required to the src/formats/date or src/formats/amount modules.
3. Rebuild the package.

## Contributing to the project

Stay tuned.