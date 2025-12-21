Transtractor
============

**Universal PDF Bank Statement Parsing**

Transtractor (Transaction Extractor) is a high-performance library for extracting transaction data from PDF bank statements. 
Built with Rust for speed and wrapped with a Python API for ease of use.

.. image:: https://github.com/transtractor/transtractor-lib/actions/workflows/tests.yml/badge.svg
   :target: https://github.com/transtractor/transtractor-lib/actions
   :alt: Tests

.. image:: https://img.shields.io/github/license/transtractor/transtractor-lib
   :target: https://github.com/transtractor/transtractor-lib/blob/main/LICENSE
   :alt: License


Quick Start
-----------

Install from PyPI:

.. code-block:: bash

   pip install transtractor

To parse a bank statement PDF and convert it to CSV:

.. code-block:: python

   from transtractor import Parser

   # Initialise parser
   parser = Parser()

   # Convert PDF to CSV
   parser.parse('statement.pdf').to_csv('statement.csv')

Writes:

.. code-block:: text

   date,description,amount,balance
   2025-01-01,Transaction 1,50000.0,100000.0
   2025-01-01,Transaction 2,-1000.0,99000.0
   2025-01-01,Transaction 3,-10000.0,89000.0
   2025-01-01,Transaction 4,1350.0,90350.0
   2025-01-03,Transaction 5,-530.99,89819.01
   2025-01-03,Transaction 6,1532.55,91351.56
   2025-01-04,Transaction 7,-568.01,90783.55
   2025-01-04,Transaction 8,-23.56,90759.99
   ...


Supported Banks
---------------
See the :doc:`supported statements <supported_statements>` page for a full list of supported banks and statement formats.
If your bank is not supported, create your own configuration by following the :doc:`guidelines <configuration>` and 
load it into the parser:

.. code-block:: python

   from transtractor import Parser

   parser = Parser()
   parser.load('path/to/your_bank_config.json')
   parser.parse('your_statement.pdf').to_csv('output.csv')


Documentation
-------------

.. toctree::
   :maxdepth: 2
   :caption: Contents:

   installation
   supported_statements
   configuration
   api_reference


Community & Support
-------------------

* **Website**: `transtractor.net <https://transtractor.net>`_
* **GitHub Repository**: `transtractor/transtractor-lib <https://github.com/transtractor/transtractor-lib>`_
* **Contributions**: Pull requests with new statement configurations are very welcome! Or email them to Daniel at `develop@transtractor.net`.


License
-------

Transtractor is open source software licensed under the MIT License.
