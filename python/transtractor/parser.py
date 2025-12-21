"""Python wrapper for the Transtractor PDF bank statement parser."""

from typing import List

from .exceptions import StatementNotSupported
from .structs.statement_data import StatementData
from .transtractor import LibParser  # pylint: disable=no-name-in-module
from .utils.default_configs import get_base_config_db
from .utils.extract import pdf_to_text_items
from .utils.testing import run_test_protocol


class Parser:
    """A PDF bank statement parser.
    
    This parser will be initialized with a set of default bank statement
    extraction configurations. When parsing a PDF, it will attempt to identify
    applicable configurations based on the text items extracted from the PDF. You
    can also load custom configurations from JSON files for additional statement
    formats.

    Example:
        parser = Parser()
        parser.load('custom_config.json')
        statement_data = parser.parse('statement.pdf')
        print(statement_data)
        statement_data.to_csv('output.csv')
    """
    def __init__(self):
        """Initialise the Parser with default configurations."""
        self._inner = LibParser()
        for key in get_base_config_db().get_all_config_keys():
            account_terms = get_base_config_db().get_account_terms(key)
            self._inner.add_account_terms(key, account_terms)

    def _identify(self, py_text_items: List[dict]) -> List[str]:
        """Identify keys applicable to the given text items. Update the
        internal config DB with any uncached configs from the base config DB.
        """
        applicable_keys = self._inner.get_applicable_config_keys(py_text_items)
        if not applicable_keys:
            raise StatementNotSupported(
                "No applicable statement extraction configuration found. Create and load a "
                "custom config then try again."
            )
        uncached_keys = self._inner.get_unregistered_config_keys(applicable_keys)
        for key in uncached_keys:
            json_str = get_base_config_db().get_config_json_str(key)
            self._inner.register_config_from_json_str(json_str)
        return applicable_keys

    def parse(self, pdf_file_path: str) -> StatementData:
        """Parse the bank statement PDF and return a StatementData object.

        :param pdf_file_path: Path to the PDF file to be processed
        :return: StatementData object representing the parsed bank statement data
        :raises NoErrorFreeStatementData: If no error-free statement data could be found
        :raises StatementNotSupported: If the statement format is unsupported or unidentifiable

        NoErrorFreeStatementData is raised in the following cases:
        
        - Quality check failure: no error-free parsed StatementData produced 
          (e.g., unbalanced transactions).
        - Missing required transaction fields: a required date/amount/balance absent.
        """
        py_text_items = pdf_to_text_items(pdf_file_path)
        applicable_keys = self._identify(py_text_items)
        sd: StatementData = self._inner.py_text_items_to_py_statement_data(
            py_text_items, applicable_keys
        )
        sd.set_filename(pdf_file_path)
        return sd

    def debug(self, pdf_file_path: str, output_file: str) -> str:
        """Write a summary of the statement data and quality checks for 
        each statement extraction configuration applied.

        :param pdf_file_path: Path to the PDF file to be processed
        :param output_file: Path to the output debug text file
        :return: The debug string written to the output file
        :raises StatementNotSupported: If the statement format is unsupported or unidentifiable
        """
        py_text_items = pdf_to_text_items(pdf_file_path)
        applicable_keys = self._identify(py_text_items)
        result = self._inner.py_text_items_to_debug_py_str(
            py_text_items, applicable_keys
        )
        with open(output_file, 'w', encoding='utf-8') as fh:
            fh.write(result)
        return result

    def layout(self, pdf_file_path: str, output_file: str, y_bin=0.0, x_gap=0.0) -> str:
        """Extract, write and return a text layout representation of the PDF page.

        :param pdf_file_path: Path to the PDF file to be processed
        :param y_bin: Y coordinate bin size for sorting/merging text items
        :param x_gap: X coordinate gap size in number of characters for merging text items
        :return: A string representing the text layout of the page
        
        Note: The values of y_bin and x_gap are same same as those used for the
        "fix_text_order" parameter in the configuration JSON files. 
        """
        py_text_items = pdf_to_text_items(pdf_file_path)
        layout_str: str = self._inner.py_text_items_to_layout_py_str(py_text_items, y_bin, x_gap)
        with open(output_file, 'w', encoding='utf-8') as fh:
            fh.write(layout_str)
        return layout_str

    def load(self, json_file_path: str) -> None:
        """Load a custom parsing configuration from a JSON file.

        Configurations loaded via this method will be registered in the
        internal configuration database and will overwrite any existing
        configuration with the same key.

        :param json_file_path: Path to the JSON configuration file
        :return: None
        :raises ConfigLoadError: If the configuration file is invalid 
            or cannot be loaded

        See the docs for detailed instructions for creating custom 
        configuration JSON files.
        """
        self._inner.import_config_from_file(json_file_path)

    def test(self, pdf_dir: str, output_file: str = "", log_level: str = "INFO") -> None:
        """Try to parse all PDFs in a given directory and sub-directories
        using the current parser configuration database. Optionally outputs 
        a CSV file summarising the test results.
        
        :param pdf_dir: Path to the directory containing PDF files to be tested
        :param output_file: Optional path to output CSV file for test results
        :param log_level: Logging level for test output (e.g., "INFO", "WARNING")
        :return: None
        
        Note: Set log_level to "WARNING" or higher to suppress terminal output.
        """
        run_test_protocol(pdf_dir, self, output_file, log_level)
