# The Transtractor

## Universal PDF bank statement parsing

The Trnasaction Extractor, or 'Transtractor', aspires to be a universal 
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

The Transtractor package will be installed in your current Python environment and ready to use.


### Basic usage (Python)

1. **Import and initialise the parser**
   ```python
   from transtractor import Parser

   parser = Parser()
   ```

2. **Convert PDF to CSV**: All CSV files are written in a standard format
   ```python
   parser.to_csv('statement.pdf', 'statement.csv')
   ```

3. **Convert PDF the dictionary**: Load into a DataFrame for analysis
   ```python
   import pandas as pd

   data = parser.to_dict('statement.pdf')
   df = pd.DataFrame(data)
   ```

An exception will be raised if your statement format is not supported or
the running balances of the extracted transactions disagree with the opening 
and closing balances in the statement.

## Unsupported statements

You may develop your own configuration settings if the Transtractor cannot 
parse your statement:

### Loading a custom configuration file

  ```python
  parser = Parser()
  parser.import_conf('my_config.json')
  parser.to_csv('statement.pdf', 'statement.csv')
  ```

### Preparing a configuration file


  ```json
{
    "key": "au__cba__credit_card__1",
    "bank_name": "Commonwealth Bank of Australia",
    "account_type": "Credit Card",
    "account_terms": ["CommBank", "Available credit"],
    "account_examples": ["Low Rate Mastercard", "Low Fee Mastercard"],
    "apply_y_patch": true,

    "opening_balance_terms": ["Opening balance"],
    "opening_balance_formats": ["format2"],
    "opening_balance_y1_tol": 2,
    "opening_balance_invert": true,

    "closing_balance_terms": ["Closing balance"],
    "closing_balance_formats": ["format2"],
    "closing_balance_y1_tol": 2,
    "closing_balance_invert": true,

    "start_date_terms": ["Statement Period"],
    "start_date_formats": ["format2"],
    "start_date_y1_tol": 2,

    "transaction_terms": ["Transactions Date Transaction Details"],
    "transaction_terms_stop": ["Please check your"],
    "transaction_formats": [
        ["date", "description", "amount"],
        ["description", "amount"]
    ],

    "transaction_date_formats": ["format1"],
    "transaction_date_headers": ["Date"],
    "transaction_date_alignment": "x1",

    "transaction_description_headers": ["Transaction Details"],
    "transaction_description_alignment": "x1",
    "transaction_description_exclude": [
        "NetBank Visit.*Transaction Details"
    ],

    "transaction_amount_formats": ["format1"],
    "transaction_amount_headers": ["Amount (A$)"],
    "transaction_amount_alignment": "x2",
    "transaction_amount_invert": true
}
  ```