"""Stub file for transtractor Rust extension module."""

from .structs.statement_data import StatementData

class LibParser:
    """Parser for extracting statement data from text items."""

    def __init__(self) -> None:
        """Create a new LibParser instance."""

    def register_config_from_json_str(self, py_json_str: str) -> None:
        """
        Register JSON configuration string into the parser database.

        :param py_json_str: JSON string containing the configuration
        :raises ConfigLoadError: If the configuration cannot be loaded
        """

    def register_config_from_file(self, py_file_path: str) -> None:
        """
        Register JSON configuration file into the parser database.

        :param py_file_path: Path to the JSON configuration file
        :raises ConfigLoadError: If the configuration file cannot be loaded
        """

    def py_pdf_path_to_py_statement_data(self, py_pdf_path: str) -> StatementData:
        """
        Process a PDF file path from Python caller and return a Python StatementData
        object.

        :param py_pdf_path: Path to the PDF file
        :raises StatementNotSupported: If no supported statement configuration is found
        :raises NoErrorFreeStatementData: If no error-free statement data could be found
        """

    def py_pdf_path_to_layout_py_str(
        self, py_pdf_path: str, x_gap: float, y_bin: float
    ) -> str:
        """
        Process a PDF file into layout text str.

        :param py_pdf_path: Path to the PDF file
        :param y_bin: Y coordinate bin size for sorting/merging text items
        :param x_gap: X coordinate gap size for merging text items
        """

    def py_pdf_path_to_debug_py_str(self, py_pdf_path: str) -> str:
        """Process a PDF file path from Python caller and return debug information as a
        string.

        :param py_pdf_path: Path to the PDF file
        """

    def layout_py_str_py_statement_data(self, py_layout_str: str) -> StatementData:
        """
        Process a layout string and return statement data as a Python object of type
        StatementData.

        :param py_layout_str: Layout string content from text file
        :raises StatementNotSupported: If no supported statement configuration is found
        :raises NoErrorFreeStatementData: If no error-free statement data could be found
        """

class NoErrorFreeStatementData(Exception):
    """Raised when no error-free statement data could be found."""

class ConfigLoadError(Exception):
    """Raised when a configuration cannot be loaded."""

class StatementNotSupported(Exception):
    """Raised when no supported statement configuration is found."""
