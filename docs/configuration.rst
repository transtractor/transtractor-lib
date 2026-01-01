Creating Configuration Files
============================

Configuration files specify the parsing parameters that the Transtractor uses to extract 
data from bank statements. This guide explains how to create your own configuration files 
for unsupported banks or account types.


Basic Template
--------------
The JSON file used to read the
`test1.pdf <https://github.com/transtractor/transtractor-lib/blob/main/tests/fixtures/test1.pdf>`_ 
example file included in the source code is:

.. code-block:: json

    {
        "key": "au__gtb__fake_account__1",
        "bank_name": "Gravy Toast Bank",
        "account_type": "Savings",
        "account_terms": ["Gravy Toast", "Fake"],
        "account_examples": ["Fake Account Product", "Similar Product"],
        "fix_text_order": [0.0, 0.0],

        "account_number_terms": ["Account number:"],
        "account_number_patterns": ["\\b\\d{4}\\s\\d{4}\\s\\d{4}\\s\\d{4}\\b"],
        "account_number_alignment": "y1",
        "account_number_alignment_tol": 5,

        "opening_balance_terms": ["Opening balance:"],
        "opening_balance_formats": ["format3"],
        "opening_balance_alignment": "y1",
        "opening_balance_alignment_tol": 5,
        "opening_balance_invert": false,

        "closing_balance_terms": ["Closing balance:"],
        "closing_balance_formats": ["format3"],
        "closing_balance_alignment": "y1",
        "closing_balance_alignment_tol": 5,
        "closing_balance_invert": false,

        "start_date_terms": ["Statement Period:"],
        "start_date_formats": ["format2"],
        "start_date_alignment": "y1",
        "start_date_alignment_tol": 5,

        "transaction_terms": ["Transaction Details"],
        "transaction_terms_stop": ["Transactions stop here."],
        "transaction_formats": [
            ["date", "description", "amount", "balance"],
            ["date", "description", "amount"],
            ["description", "amount", "balance"],
            ["description", "amount"]
        ],
        "transaction_new_line_tol": 5,
        "transaction_start_date_required": true,
        "transaction_alignment_tol": 10,

        "transaction_date_formats": ["format1"],
        "transaction_date_headers": ["Date"],
        "transaction_date_alignment": "x1",

        "transaction_description_headers": ["Description"],
        "transaction_description_alignment": "x1",
        "transaction_description_exclude": [
            " Annoying text",
            " text to filter out"
        ],

        "transaction_amount_formats": ["format1", "format2"],
        "transaction_amount_headers": ["Credit"],
        "transaction_amount_alignment": "x2",
        "transaction_amount_invert_headers": ["Debit"],
        "transaction_amount_invert_alignment": "x2",
        "transaction_amount_invert": false,

        "transaction_balance_formats": ["format4"],
        "transaction_balance_headers": ["Balance"],
        "transaction_balance_alignment": "x2",
        "transaction_balance_invert": false
    }



How Parsing Works
-----------------
The first step in parsing is to extract all text elements from the PDF file along with their
positions on the page. The Transtractor then sequentially reads though each item and extracts
information based on sequential, positional, and formatting rules defined in the configuration 
file. 

For a view of what the Transtractor "sees" when parsing a PDF file, you can extract the PDF into
`layout text` by:

.. code-block:: python

    from transtractor import Parser
    parser = Parser()
    parser.layout('test1.pdf', 'test1_layout.txt')

The first few lines of the resulting `test1_layout.txt` file will look like:

.. code-block:: text

    [Page 0]
    ["Gravy",72,101,49,37]["Toast",104,131,49,37]["Bank",133,159,49,37]
    ["Fake",77,109,88,74]["Monthly",113,166,88,74]["Statement",170,238,88,74]
    ["Statement",77,131,119,107]["Period:",135,173,119,107]["1",268,275,119,107]["Jan",278,298,119,107]["2025",301,328,119,107]["to",331,341,119,107]["31",344,358,119,107]["Jan",361,380,119,107]["2025",383,410,119,107]
    ["Opening",77,122,134,122]["balance:",125,171,134,122]["$50,000.00",268,328,134,122]["CR",332,349,134,122]
    ["Closing",77,117,149,137]["balance:",120,165,149,137]["$11,663.82",268,328,149,137]["CR",332,349,149,137]
    ["Account",77,120,164,152]["number:",123,167,164,152]["1234",268,295,164,152]["5678",298,325,164,152]["9123",328,355,164,152]["4567",358,385,164,152]
    ["Transaction",77,156,200,186]["Details",160,206,200,186]
    ["Date",77,103,221,209]["Description",149,215,221,209]["Debit",298,328,221,209]["Credit",365,399,221,209]["Balance",456,502,221,209]
    ["01",77,90,239,227]["Jan",94,113,239,227]["Transaction",149,211,239,227]["1",215,222,239,227]["50,000.00",346,399,239,227]
    ["Transaction",149,211,256,244]["2",215,222,256,244]["1,000.00",281,328,256,244]
    ["Transaction",149,211,273,261]["3",215,222,273,261]["10,000.00",275,328,273,261]
    ["Transaction",149,211,289,277]["4",215,222,289,277]["1,350.00",352,399,289,277]["90,350",445,481,289,277]["CR",485,502,289,277]
    ["03",77,90,306,294]["Jan",94,113,306,294]["Transaction",149,211,306,294]["5",215,222,306,294]["530.99",291,328,306,294]

Each text element is represented as ["text",x1,x2,y1,y2], where `text` is the extracted text,
`x1` and `x2` are the horizontal positions of the start and end of the text, and `y1` and `y2` are the
bottom and top vertical positions of the text.


Format Parameters
-----------------
The Transtractor uses pattern recognition and additional logic to parse amounts and dates
from text into a standardised format. Applicable formats are specified in the configuration
file (e.g., *_formats* fields). The formats currently supported are listed below. Contact 
the project maintainers if you need additional formats, or sumit a pull request with your
contributions (see below).


Amount/Balance Formats
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
All amount or balance formats convert text into a decimal number with two decimal places. The following
formats are supported:


.. list-table::
   :header-rows: 1
   :widths: 20 80

   * - Label
     - Examples
   * - ``format1``
     - "1,234.56" → 1234.56, "-1,234.56" → -1234.56, "1,234.56-" → -1234.56
   * - ``format2``
     - "-$1,234.56" → -1234.56, "$1,234.56" → 1234.56, "$1,234.56-" → -1234.56
   * - ``format3``
     - "$1,234.56 CR" → 1234.56, "-$1,234.56 CR" → -1234.56, "$1,234.56 DR" → -1234.56
   * - ``format4``
     - "1,234.56 CR" → 1234.56, "-1,234.56 CR" → -1234.56, "1,234.56 DR" → -1234.56
   * - ``format5``
     - "nil" → 0.00, "Nil" → 0.00

Formats are sensitive to spacing and comma separation, but generally not case sensitive.


Date Formats
~~~~~~~~~~~~~~~~~~~~~~~~
Date formats convert text into the standard ISO format of YYYY-MM-DD. The following
formats are supported:

.. list-table::
   :header-rows: 1
   :widths: 20 80

   * - Label
     - Examples
   * - ``format1``
     - "24 Mar" → XXXX-03-24, "24 mar" → XXXX-03-24, "24 March" → XXXX-03-24
   * - ``format2``
     - "24 march 2025" → 2025-03-24, "24 Mar 2025" → 2025-03-24
   * - ``format3``
     - "mar 24, 2025" → 2025-03-24, "March 24, 2025" → 2025-03-24
   * - ``format4``
     - "24/3/2020" → 2020-03-24, "24/3/2020" → 2020-03-24
   * - ``format5``
     - "24/3/25" → 2025-03-24, "24/03/25" → 2025-03-25
   * - ``format6``
     - "3/24" → XXXX-03-24, "03/24" → XXXX-03-24
   * - ``format7``
     - "24-03-2023" → 2023-03-24, "24-3-2023" → 2023-03-24, "24-03-23" → 2023-03-24, "24-3-23" → 2023-03-24

Formats with a "XXXX" year will infer the year based on the statement start date.


Add New Formats
~~~~~~~~~~~~~~~~~
New formats must be added to the Rust source code *src/formats/amount* or 
*src/formats/date* directory. Add your new format parser as a new module and update the
register it in the mod.rs file. Please follow the existing code structure and include unit tests
for your new format. Re-compile with Maturin to apply changes.

Pull requests are welcome!


Extraction Parameters
---------------------
The parser extracts the *Account Number*, *Start Date*, *Opening and Closing Balances*,
and the tabulated *Transactions* from the statement. The *Transactions* must include at least
the *Date*, *Description*, and *Amount* fields. *Balance* is optional. Implicit values are filled
automatically (e.g., missing balances, shared dates).The extraction parameters used in the 
configuration file are described below.


General Statement Parameters
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
These parameters are used to identify the statement and configuration setting,
and control any pre-processing of the text before extraction begins.

*key*
****
Descriptive unique identifier for the configuration file. This must follow the format 
``<country_code>__<bank_code>__<account_type>__<version>``. The country_code
must be a valid lowercase 
`ISO 3166-1 alpha-2 country code <https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2>`_. 
The bank_code is ideally the stock ticker symbol or a commonly used abbreviation for the bank, 
but can be another short lowercase string. The account_type is a descriptive short lowercase
string such as "debit", "credit_card", or "loan". The version is an integer starting from 1
that is incremented for each new version of the configuration file.

*bank_name*
*****************
Full name of the bank. Max. 100 characters.

*account_type*
*********************
Must be one of:

- "Checking"
- "Savings"
- "Credit Card"
- "Loan"
- "Mortgage"
- "Investment"
- "Mixed"
- "Other"

*account_terms*
*********************
List of terms that distinguish this statement from the statements
from other banks or account types from the same bank. The statement
must have all these terms present to be considered a match. If a statement
matches the terms of multiple configuration files, then all configuration files
will be tried in sequence until one successfully parses the statement.

*account_examples*
************************
List of example account product names that this configuration file is intended to support.
Many banks will often have multiple account products with similar statement formats. For example,
the Commonwealth Bank of Australia's "Smart Access", "Streamline" and "Everyday Offset" accounts 
use the same statement format.

*fix_text_order*
************************
List of two float values *y_bin* and *x_gap* used to adjust the text ordering when extracting
text from the PDF file. The *y_bin* value (typically half the character height)
groups text items with *y1* positions within *y_bin* of each other into the same line, then 
orders them by their *x1* positions. Items with *x2* and *x1* postions within *x_gap* will 
be grouped together. *x_gap* is a multiplier of the average character width based on the 
preceding text item.

*y_bin*-based reordering is helpful where text items are not ordered as would be expected
based on their visual appearance in the statement. Some cases of this can make the statement
un-parsable without this adjustment.

*x_gap*-based merging is useful for filtering out header and footer text that sometimes 
find their way into transaction descriptions as individually they may satisfy the horizontal
alignment requirements, but when merged with adjacent text items they will not.

Set *y_bin* to 0.0 to disable reordering and merging. Or just set *x_gap* to 0.0 to disable merging,
but still enable reordering. Only use this parameter if absolutely necessary as it may reduce
parsing performance and add complexity to the parsing process.


Account Number Parameters
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
These parameters are used to identify and extract the account number from the statement.

*account_number_terms*
*******************************
List of text terms that appear before or above the account number. This will prime
the parser to start scanning for an account number satisfying one of the 
*account_number_patterns*. The parser ill stop trying to find the account number once
it is set. The parser only requires one of these terms to be present to start searching
for the account number.

*account_number_patterns*
****************************************
List of regular expression patterns that match the account number format. The parser 
will try to match each pattern in sequence until one is found.

*account_number_alignment*
*************************************
Specifies the alignment of the account number relative to the *account_number_terms*.
Must be one of "x1", "x2", "y1", "y2" or "". For example, if set to "y1", then the
account number must be horizontally aligned with the *account_number_terms*. 
If set to "", then no alignment checking will be performed and the first matching
account number found after the *account_number_terms* will be used.

*account_number_alignment_tol*
*****************************************
Integer value specifying the tolerance (in points) for alignment checking of the
account number. For example, if *account_number_alignment* is "y1" and this value is 5,
then the *y1* position of the account number must be within 5 points of the *y1* position
of the *account_number_terms*.


Opening Balance Parameters
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
These parameters are used to identify and extract the opening and closing balances
from the statement.

*opening_balance_terms*
*************************************
List of text terms that appear before or above the opening balance. This will prime
the parser to start scanning for the opening balance. The parser will stop trying to find
the opening balance once it is set. The parser only requires one of these terms to be present
to start searching for the opening balance.

*opening_balance_formats*
****************************************
List of amount formats (see above) that the opening balance may be in. The parser will try to
parse each format in sequence until one is successful.

*opening_balance_alignment*
***************************************
Specifies the alignment of the opening balance relative to the *opening_balance_terms*.
Must be one of "x1", "x2", "y1", "y2" or "". For example, if set to "y1", then the
opening balance must be horizontally aligned with the *opening_balance_terms*. 
If set to "", then no alignment checking will be performed and the first matching
opening balance found after the *opening_balance_terms* will be used.

*opening_balance_alignment_tol*
******************************************
Integer value specifying the tolerance (in points) for alignment checking of the
opening balance. For example, if *opening_balance_alignment* is "y1" and this value is 5,
then the *y1* position of the opening balance must be within 5 points of the *y1* position
of the *opening_balance_terms*.

*opening_balance_invert*
*************************************
Boolean value specifying whether to invert the sign of the extracted opening balance. This is
often useful for loan or credit card statements where the opening balance is presented as a
positive value despite it being a liability.


Closing Balance Parameters
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
These parameters are used to identify and extract the closing balance from the statement.

*closing_balance_terms*
*************************************
List of text terms that appear before or above the closing balance. This will prime
the parser to start scanning for the closing balance. The parser will stop trying to find
the closing balance once it is set. The parser only requires one of these terms to be present
to start searching for the closing balance.

*closing_balance_formats*
****************************************
List of amount formats (see above) that the closing balance may be in. The parser will try to
parse each format in sequence until one is successful.

*closing_balance_alignment*
***************************************
Specifies the alignment of the closing balance relative to the *closing_balance_terms*.
Must be one of "x1", "x2", "y1", "y2" or "". For example, if set to "y1", then the
closing balance must be horizontally aligned with the *closing_balance_terms*. 
If set to "", then no alignment checking will be performed and the first matching
closing balance found after the *closing_balance_terms* will be used.

*closing_balance_alignment_tol*
******************************************
Integer value specifying the tolerance (in points) for alignment checking of the
closing balance. For example, if *closing_balance_alignment* is "y1" and this value is 5,
then the *y1* position of the closing balance must be within 5 points of the *y1* position
of the *closing_balance_terms*.

*closing_balance_invert*
*************************************
Boolean value specifying whether to invert the sign of the extracted closing balance. This is
often useful for loan or credit card statements where the closing balance is presented as a
positive value despite it being a liability.


Start Date Parameters
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
These parameters are used to identify and extract the statement start date from the statement.
This date is used to infer missing years in transaction dates.

*start_date_terms*
*************************************
List of text terms that appear before or above the statement start date. This will prime
the parser to start scanning for the start date. The parser will stop trying to find
the start date once it is set. The parser only requires one of these terms to be present
to start searching for the start date.

*start_date_formats*
****************************************
List of date formats (see above) that the start date may be in. The parser will try to
parse each format in sequence until one is successful.

*start_date_alignment*
***************************************
Specifies the alignment of the start date relative to the *start_date_terms*.
Must be one of "x1", "x2", "y1", "y2" or "". For example, if set to "y1", then the
start date must be horizontally aligned with the *start_date_terms*. 
If set to "", then no alignment checking will be performed and the first matching
start date found after the *start_date_terms* will be used.

*start_date_alignment_tol*
******************************************
Integer value specifying the tolerance (in points) for alignment checking of the
start date. For example, if *start_date_alignment* is "y1" and this value is 5,
then the *y1* position of the start date must be within 5 points of the *y1* position
of the *start_date_terms*.


Transaction Parameters
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
These parameters are used to identify and extract the transaction table from the statement.

*transaction_terms*
*************************************
List of text terms that indicate the start of the transaction table. The parser will start
looking for transactions after these terms are found. The parser only requires one of these
terms to be present to start searching for transactions.

*transaction_terms_stop*
*************************************
List of text terms that indicate the end of the transaction table. The parser will stop
looking for transactions once these terms are found. The parser only requires one of these
terms to be present to stop searching for transactions.

*transaction_formats*
****************************************
List of expected transaction field arrangements. Each arrangement is a list of field names
from the set: "date", "description", "amount", "balance". This allows the parser to know when
to start and stop reading fields for each transaction, and recognised when a transaction is 
complete.

*transaction_new_line_tol*
******************************************
Integer value specifying the tolerance (in points) for detecting new lines in the transaction
descriptions. Aim for approx. 50% of the average character height in the transaction table.

*transaction_start_date_required*
******************************************
Boolean value specifying whether the start date is required for parsing transactions. Set as true
if transaction dates do not specify the year and need to be inferred from the statement start date.

*transaction_alignment_tol*
******************************************
Integer value specifying the tolerance (in points) for alignment checking of the
transaction fields and the field headers.

*transaction_date_formats*
****************************************
List of date formats (see above) that transaction dates may be in. The parser will try to
parse each format in sequence until one is successful.

*transaction_date_headers*
****************************************
List of text headers that identify the transaction date column. The parser will use these
to identify the horizontal position of the date field in the transaction table.

*transaction_date_alignment*
****************************************
Specifies the alignment of the transaction date field relative to the *transaction_date_headers*.
Must be one of "x1" (left-aligned) or "x2" (right-aligned).

*transaction_description_headers*
****************************************
List of text headers that identify the transaction description column. The parser will use these
to identify the horizontal position of the description field in the transaction table.

*transaction_description_alignment*
****************************************
Specifies the alignment of the transaction description field relative to the *transaction_description_headers*.
Must be one of "x1" (left-aligned) or "x2" (right-aligned).

*transaction_description_exclude*
****************************************
List of regex patterns to identify and remove unwanted text in transaction descriptions. This is
useful for filtering out recurring header or footer text that may appear in the transaction
descriptions, dot leaders, or other unwanted text.

*transaction_amount_formats*
****************************************
List of amount formats (see above) that transaction amounts may be in. The parser will try to
parse each format in sequence until one is successful.

*transaction_amount_headers*
****************************************
List of text headers that identify the transaction amount column. The parser will use these
to identify the horizontal position of the amount field in the transaction table. If there are
separate debit and credit columns, then set this to the credit column header.

*transaction_amount_alignment*
****************************************
Specifies the alignment of the transaction amount field relative to the *transaction_amount_headers*.
Must be one of "x1" (left-aligned) or "x2" (right-aligned).

*transaction_amount_invert_headers*
****************************************
List of text headers that identify transaction amount columns where the sign needs to be inverted.
For example, if there are separate debit and credit columns, then set this to the debit column header.

*transaction_amount_invert_alignment*
****************************************
Specifies the alignment of the transaction amount field relative to the *transaction_amount_invert_headers*.
Must be one of "x1" (left-aligned) or "x2" (right-aligned).

*transaction_amount_invert*
*************************************
Boolean value specifying whether to invert the sign of the extracted transaction amounts. This is
often useful for loan or credit card statements where debits are presented as positive values
despite being liabilities.

*transaction_balance_formats*
****************************************
List of amount formats (see above) that transaction balances may be in. The parser will try to
parse each format in sequence until one is successful. Leave empty if transaction balances are not
present in the statement.

*transaction_balance_headers*
****************************************
List of text headers that identify the transaction balance column. The parser will use these
to identify the horizontal position of the balance field in the transaction table. Leave empty
if transaction balances are not present in the statement.

*transaction_balance_alignment*
****************************************
Specifies the alignment of the transaction balance field relative to the *transaction_balance_headers*.
Must be one of "x1" (left-aligned) or "x2" (right-aligned). Cannot be left empty.

*transaction_balance_invert*
*************************************
Boolean value specifying whether to invert the sign of the extracted transaction balances. This is
often useful for loan or credit card statements where balances are presented as positive values
despite being liabilities.


Testing Your Configuration
--------------------------------------
Once you have created your configuration file, you can test it by loading it into the
Transtractor parser and parsing a sample statement:

.. code-block:: python

   from transtractor import Parser

   # Initialise parser with your configuration file
   parser = Parser()
   parser.load('your_config_file.json')
   parser.parse('sample_statement.pdf').to_csv('sample_statement.csv')

Even better, try it out against all your bank statements to ensure it works across
multiple years and is tolerant of edge cases:

.. code-block:: python

   from transtractor import Parser

   parser = Parser()
   parser.load('your_config_file.json')
   parser.test('directory_containing_statements', 'test_results.csv')

This will recursively parse all PDF statements in the directory and sub-directories, and
output a CSV file with the results. Review the results to ensure all statements were parsed
correctly.

Troubleshooting
----------------------
Here are some common issues you may encounter when creating configuration files,
and how to resolve them.

Zero-Balances
^^^^^^^^^^^^^^^^^^^^
Somtimes statements will show zero balances as "nil", "zero", or similar text.
Ensure you include *format5* in the relevant *_formats* fields to handle these cases.
This is a common case for you first statement of a new account.

Hidden Characters
^^^^^^^^^^^^^^^^^^^^^^^^^^^^
PDFs may contain characters that are not visible when viewing the statement, but
are extracted by the parser. These hidden characters can interfere with pattern matching.
To identify hidden characters, extract the layout text using the `layout` method
(as described above) and inspect the text around the problematic areas. You may need
to adjust your regex patterns to account for these hidden characters.

Unexpected Text Order
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
If the parser is not finding expected terms or fields, the text order extracted
from the PDF may not match the visual order. Use the *fix_text_order* parameter
to adjust the text ordering based on *y_bin* and *x_gap* values. Experiment
with different values to achieve the correct ordering. inspect the layout text
to understand how the text items are ordered.

Missing Date or Amount Formats
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
The Transtractor is still growing its library of supported date and amount formats. If you encounter
a date or amount format that is not recognised, you may need to add a new format parser
to the Rust source code (as described above). Contact the project maintainers if you need
assistance with this process.


Contributing Your Configuration
--------------------------------------
If you have created a well-tested configuration file for a bank or account type that is not
currently supported, please consider contributing it to the project! You can submit a
pull request on the
`GitHub repository <https://github.com/transtractor/transtractor-lib>`_, including your configuration
file in the *python/transtractor/configs* directory. Otherwise, feel free to email it to the project maintainers
for inclusion (develop@transtractor.net).
