Supported Statements
====================

The Transtractor uses rules-based parsing to extract transaction data from bank statements. Each 
supported statement format is defined by a specific set of parsing rules tailored to the bank 
and account type. These files are located in the `python/transtractor/configs` directory of 
the `source code <https://github.com/transtractor/transtractor-lib>`_.

The following statements will be recognised and parsed authomatically. You must create and load 
your own configuration files if your bank or account type is not listed here.

Australia
---------

.. list-table::
   :header-rows: 1
   :widths: 15 40 30 15

   * - Key
     - Bank
     - Types
     - Introduced
   * - ``au__cba__credit_card__1``
     - Commonwealth Bank
     - Credit Card
     - v0.9.0
   * - ``au__cba__debit__1``
     - Commonwealth Bank
     - Debit/Savings
     - v0.9.0
   * - ``au__cba__loan__1``
     - Commonwealth Bank
     - Loan
     - v0.9.0
   * - ``au__nab__classic_banking__1``
     - National Australia Bank
     - Classic Banking
     - v0.9.0