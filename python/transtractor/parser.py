from typing import List

from .transtractor import LibParser, NoErrorFreeStatementData
from .structs.statement_data import StatementData
from .utils.extract import pdf_to_text_items
from .utils.default_configs import get_base_config_db
import time
from pathlib import Path

class StatementNotSupported(Exception):
    """Raised when the statement format is unsupported or unidentifiable."""


class Parser:
    """A bank statement parser capable of identifying statement types
    and converting them to CSV.
    
    The parser is pre-configured to recognise and extract data 
    from various bank statement PDFs. It can also be extended 
    with custom parsing configurations in cases where a statement format
    is not recognised by the core configurations.

    Example:
        ...
    """
    def __init__(self):
        self._inner = LibParser()
        # Load typer with terms
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
        :return: StatementData object representing the parsed bank statement
        :raises NoErrorFreeStatementData: If no error-free statement data could be found
        :raises StatementNotSupported: If the statement format is unsupported or unidentifiable

        NoErrorFreeStatementData is raised in the following cases:
        - Quality check failure: no error-free parsed StatementData produced (e.g., unbalanced transactions).
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
        """Extract and return a text layout representation of the PDF page.

        :param pdf_file_path: Path to the PDF file to be processed
        :param y_bin: Y coordinate bin size for sorting/merging text items
        :param x_gap: X coordinate gap size for merging text items
        :return: A string representing the text layout of the page
        """
        py_text_items = pdf_to_text_items(pdf_file_path)
        layout_str: str = self._inner.py_text_items_to_layout_py_str(py_text_items, y_bin, x_gap)
        with open(output_file, 'w', encoding='utf-8') as fh:
            fh.write(layout_str)
        return layout_str

    def test(self, pdf_dir: str) -> None:
        """Detect all PDFs in a given directory and sub-directories
        and attempt to parse them. Reports results to stdout.

        :param pdf_dir: Path to the directory containing PDF files to be tested
        :return: None
        
        Output format:
            Testing {file_path}...
            Transactions={#}\t'Key={config_keys}'\tTime={time}ms\tStatus={PASS|FAIL}
            
            Transactions: Number of successfully parsed transactions. 'Check Debug' if FAIL.
            Key: The config keys used for parsing. 'None' if no applicable config found.
            Time: Time taken to parse in milliseconds
            Status: PASS if parsing was successful and error-free, FAIL otherwise
        """        
        # Get all PDF files in the directory and sub-directories
        pdf_files: list[str] = [str(p) for p in Path(pdf_dir).rglob("*.pdf")]
        
        for pdf_file in pdf_files:
            print(f"Testing {pdf_file}...")
            num_transactions = 0
            keys = ""
            status = "FAIL"
            start_time = time.time()
            try:
                py_text_items = pdf_to_text_items(pdf_file)
                keys = self._identify(py_text_items)
                sd: StatementData = self._inner.py_text_items_to_py_statement_data(
                    py_text_items, keys
                )
                num_transactions = len(sd.transactions)
                status = "PASS"
            except StatementNotSupported:
                keys = "None"
            except NoErrorFreeStatementData:
                pass
            end_time = time.time()
            elapsed_ms = int((end_time - start_time) * 1000)
            print(
                f"Transactions={num_transactions}\tKey={keys}\t"
                f"Time={elapsed_ms}ms\tStatus={status}"
            )
