"""Python wrapper for the Transtractor PDF bank statement parser."""

from typing import cast

from .structs.statement_data import StatementData
from .transtractor import LibParser
from .utils.testing import run_test_protocol


class Parser:
    """A PDF bank statement parser.

    This parser will be initialised with a set of default bank statement
    extraction configuration database. When parsing a PDF, it will attempt to identify
    applicable configurations based on keywords extracted from the PDF. You
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
        """Initialise the Parser with default database."""
        self._inner = LibParser()

    def parse(self, pdf_file_path: str) -> StatementData:
        """Parse the bank statement PDF and return a StatementData object.

        :param pdf_file_path: Path to the PDF file to be processed
        :return: StatementData object representing the parsed bank statement data
        :raises NoErrorFreeStatementData: Statement format recognised but could be
            processed without failing quality checks
        :raises StatementNotSupported: Statement format is unsupported or not properly
            identified
        """
        sd: StatementData = cast(
            StatementData, self._inner.py_pdf_path_to_py_statement_data(pdf_file_path)
        )
        sd.set_filename(pdf_file_path)
        return sd

    def parse_layout(self, layout_file_path: str) -> StatementData:
        """Parse the bank statement layout string and return a StatementData object.

        :param layout_file_path: Path to the layout file to be processed
        :return: StatementData object representing the parsed bank statement data
        :raises NoErrorFreeStatementData: Statement format recognised but could be
            processed without failing quality checks
        :raises StatementNotSupported: Statement format is unsupported or not properly
            identified
        """
        py_layout_str = open(layout_file_path, encoding="utf-8").read()
        sd: StatementData = cast(
            StatementData, self._inner.layout_py_str_py_statement_data(py_layout_str)
        )
        return sd

    def debug(self, pdf_file_path: str, output_file: str) -> str:
        """Write a summary of the statement data and quality checks for
        each statement extraction configuration applied.

        :param pdf_file_path: Path to the PDF file to be processed
        :param output_file: Path to the output debug text file
        :return: The debug string written to the output file
        :raises StatementNotSupported: Statement format is unsupported or not properly
            identified
        """
        result = self._inner.py_pdf_path_to_debug_py_str(pdf_file_path)
        with open(output_file, "w", encoding="utf-8") as fh:
            fh.write(result)
        return result

    def layout(self, pdf_file_path: str, output_file: str, y_bin=0.0, x_gap=0.0) -> str:
        """Extract, write and return a text layout representation of the PDF page.

        :param pdf_file_path: Path to the PDF file to be processed
        :param y_bin: Y coordinate bin size for sorting/merging text items
        :param x_gap: X coordinate gap size in number of characters for merging text
            items
        :return: A string representing the text layout of the page

        Note: The values of y_bin and x_gap are same same as those used for the
        "fix_text_order" parameter in the configuration JSON files.
        """
        layout_str: str = self._inner.py_pdf_path_to_layout_py_str(
            pdf_file_path, y_bin, x_gap
        )
        with open(output_file, "w", encoding="utf-8") as fh:
            fh.write(layout_str)
        return layout_str

    def load(self, json_file_path: str) -> None:
        """Load a custom parsing configuration from a JSON file.

        Configurations loaded via this method will be registered in the
        internal database and will overwrite any existing configuration with the same
        key.

        :param json_file_path: Path to the JSON configuration file
        :return: None
        :raises ConfigLoadError: Configuration file is invalid or cannot be loaded

        See the docs for detailed instructions for creating custom
        configuration JSON files.
        """
        self._inner.register_config_from_file(json_file_path)

    def test(
        self, pdf_dir: str, output_file: str = "", log_level: str = "INFO"
    ) -> None:
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
