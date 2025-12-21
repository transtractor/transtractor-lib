# The Transtractor

![Tests](https://github.com/transtractor/transtractor-lib/actions/workflows/tests.yml/badge.svg)
![License](https://img.shields.io/github/license/transtractor/transtractor-lib)

## Universal PDF bank statement parsing
The Transaction Extractor, or 'Transtractor', aspires to be a universal 
library for extracting transaction data from PDF bank statements. Key features:

* Written in Rust (fast)
* Python API (user friendly)
* AI-free (lightweight)
* Rules-based extraction (100% predictable and accurate)


## Installation

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

3. **Convert PDF to DataFrame**: Load into a DataFrame for analysis
   ```python
   import pandas as pd

   data = parser.parse('statement.pdf').to_pandas_dict()
   df = pd.DataFrame(data)
   ```

## Supported statements
See the documentation for a current list of [supported statements](https://transtractor-lib.readthedocs.io/en/latest/supported_statements.html). You may also
create your own parsing configuration files by following these [instructions](https://transtractor-lib.readthedocs.io/en/latest/configuration.html)
and loading it by:

```python
from transtractor import Parser

parser = Parser()
parser.load('my_config.json')
parser.parse('statement.pdf').to_csv('statement.csv')
```

## Contributions
New and well-tested configuration files are especially welcome. Please
submit a pull request with them add to the *data/configs* directory, or
email to develop@transtractor.net.
