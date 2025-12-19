"""Testing utilities for the Transtractor parser module."""

import csv
import logging
import time
from pathlib import Path
from typing import TYPE_CHECKING, List

from ..exceptions import StatementNotSupported
from ..structs.statement_data import StatementData
from ..transtractor import NoErrorFreeStatementData # pylint: disable=no-name-in-module
from .extract import pdf_to_text_items

if TYPE_CHECKING:
    from ..parser import Parser


class TestData:
    """Class for managing test data files and directories."""

    # pylint: disable=too-many-instance-attributes
    def __init__(self, pdf_file_path: str, parser: 'Parser'):
        self.pdf_file_path = pdf_file_path # The PDF file being tested
        self.parser = parser  # The Parser instance used for testing
        self.num_pages: int = 0  # Number of pages in the PDF
        self.num_transactions: int = 0  # Number of transactions extracted
        self.config_keys: str = ""  # Config keys used for parsing
        self.extract_time: int = 0  # Time taken to extract text items from PDF
        self.identify_time: int = 0  # Time taken to identify config keys
        self.parse_time: int = 0  # Time taken for parsing in ms
        self.total_time: int = 0  # Total time taken for the test in ms
        self.status: str = ""  # Status of the test (PASS/FAIL)
        self.reason_failed: str = ""  # Error message if any

    @staticmethod
    def get_header_all() -> List[str]:
        """Get all headers for writing CSV file."""
        return [
            "PDF File",
            "Pages",
            "Transactions",
            "Config Keys",
            "Extract Time (ms)",
            "Identify Time (ms)",
            "Parse Time (ms)",
            "Total Time (ms)",
            "Status",
            "Reason Failed"
        ]

    @staticmethod
    def get_header_log() -> List[str]:
        """Get headers for writing log file."""
        return [
            "Status",
            "Transactions",
            "Total Time (ms)",
            "PDF File",
        ]

    def get_all(self) -> List[str]:
        """Get all data fields as strings for writing CSV file."""
        return [
            self.pdf_file_path,
            str(self.num_pages),
            str(self.num_transactions),
            self.config_keys,
            str(self.extract_time),
            str(self.identify_time),
            str(self.parse_time),
            str(self.total_time),
            self.status,
            self.reason_failed
        ]

    def get_log(self) -> List[str]:
        """Get data fields as strings for writing log file."""
        return [
            self.status,
            str(self.num_transactions),
            str(self.total_time),
            self.pdf_file_path,
        ]

    # pylint: disable=protected-access
    def run(self) -> None:
        """Run the test on the PDF file using the provided parser."""
        start_total = time.time()

        # Extract text items
        start_extract = time.time()
        py_text_items = pdf_to_text_items(self.pdf_file_path)
        end_extract = time.time()
        self.extract_time = int((end_extract - start_extract) * 1000)
        if py_text_items:
            self.num_pages = py_text_items[-1]["page"] - py_text_items[0]["page"] + 1

        # Try to identify config keys to use
        start_identify = time.time()
        try:
            keys = self.parser._identify(py_text_items)
        except StatementNotSupported:
            self.status = "FAIL"
            self.reason_failed = "StatementNotSupported"
            keys = ""
        end_identify = time.time()
        self.identify_time = int((end_identify - start_identify) * 1000)
        self.config_keys = ", ".join(keys)
        if keys == "":
            end_total = time.time()
            self.total_time = int((end_total - start_total) * 1000)
            return

        # Try to parse the statement
        start_parse = time.time()
        try:
            sd: StatementData = self.parser._inner.py_text_items_to_py_statement_data(
                py_text_items, keys
            )
            self.num_transactions = len(sd.transactions)
            self.status = "PASS"
        except NoErrorFreeStatementData:
            self.status = "FAIL"
            self.reason_failed = "NoErrorFreeStatementData"
        end_parse = time.time()
        self.parse_time = int((end_parse - start_parse) * 1000)

        end_total = time.time()
        self.total_time = int((end_total - start_total) * 1000)


# pylint: disable=too-many-locals
def run_test_protocol(
    pdf_dir: str,
    parser: 'Parser',
    output_file: str = "",
    log_level: str = "INFO"
) -> List[TestData]:
    """Run test protocol on all PDFs in a given directory and sub-directories.
    
    :param pdf_dir: Path to the directory containing PDF files to be tested
    :param parser: Parser instance to use for testing
    :param output_file: Optional path to output CSV file for test results
    :param log_level: Logging level (DEBUG, INFO, WARNING, ERROR, CRITICAL)
    
    Note: Set log_level to "WARNING" or higher to suppress terminal output.
    """
    # Set up logging
    numeric_level = getattr(logging, log_level.upper(), None)
    if not isinstance(numeric_level, int):
        raise ValueError(f"Invalid log level: {log_level}")
    logging.basicConfig(level=numeric_level, format='%(message)s')
    logger = logging.getLogger()

    # Get all PDF files in the directory and sub-directories
    pdf_files: List[str] = [str(p) for p in Path(pdf_dir).rglob("*.pdf")]
    num_files = len(pdf_files)
    num_passed = 0
    num_failed = 0
    test_results: List[TestData] = []
    log_header = "\t".join(["Test"] + TestData.get_header_log())
    logger.info(log_header)

    file_count = 1
    for pdf_file in pdf_files:
        test_data = TestData(pdf_file, parser)
        test_data.run()
        test_results.append(test_data)
        logger.info(
            "%s/%s\t%s\t%s\t%sms\t%s",
            file_count,
            num_files,
            test_data.status,
            test_data.num_transactions,
            test_data.total_time,
            test_data.pdf_file_path
        )
        if test_data.status == "PASS":
            num_passed += 1
        else:
            num_failed += 1
        file_count += 1

    # Write results to output CSV file if specified
    if output_file:
        with open(output_file, mode='w', newline='', encoding='utf-8') as csvfile:
            writer = csv.writer(csvfile)
            writer.writerow(TestData.get_header_all())
            for result in test_results:
                writer.writerow(result.get_all())

    logger.info("Summary: %s passed, %s failed out of %s files.", num_passed, num_failed, num_files)

    return test_results
