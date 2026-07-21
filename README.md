# The Transtractor

![PyPI version](https://img.shields.io/pypi/v/transtractor)
![Development Status](https://img.shields.io/pypi/status/transtractor)
![Tests](https://github.com/transtractor/transtractor-lib/actions/workflows/tests.yml/badge.svg)
![codecov](https://codecov.io/gh/transtractor/transtractor-lib/branch/main/graph/badge.svg)
![License](https://img.shields.io/github/license/transtractor/transtractor-lib)

## Universal PDF Bank Statement Parsing
The Transaction Extractor, or 'Transtractor', aspires to be a universal 
library for extracting transaction data from PDF bank statements. Key features:

* Written in Rust (fast)
* Python API (user friendly)
* AI-free (lightweight)
* Rules-based extraction (100% predictable and accurate)


## Installation
### Install from PyPI
Transtractor is available on PyPI and can be installed with pip:

```shell
pip install transtractor
```

### Compile from Source
1. **Install Rust**: Download and install Rust from [rustup.rs](https://rustup.rs/)

2. **Install uv**: Follow instructions from [Astral](https://docs.astral.sh/uv/getting-started/installation/)

3. **Sync Python environment and compile**: Clone the repository and build
   ```shell
   git clone https://github.com/transtractor/transtractor.git
   cd transtractor-lib
   uv sync --locked --group dev
   ```

4. **Test the package**: Run Rust and Python unit tests
   ```shell
   cargo test
   uv run pytest
   ```

### Basic Usage
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

## Advanced Usage
See the [documentation](https://transtractor-lib.readthedocs.io/en/latest/) maintained on Read the Docs.

## Supported Statements
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
submit a pull request with them add to the `python/transtractor/configs` directory, or
email to gravytoast@pm.me.

The following pages provide further information about how this package is built and developed:

* [Architecture Guide](md/architecture.md): Overview of key application components and design principles.
* [Developer Guide](md/develop.md): Reference page for core development and maintenance.