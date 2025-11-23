from .transtractor import LibParser
from .utils.write import dict_to_csv
from .utils.extract import pdf_to_text_items


class Parser:
    """
    A bank statement parser that can identify statement types and convert them to CSV format.
    
    The parser supports both PDF and TXT files (layout text format) and uses predefined 
    configurations to identify different bank statement formats. It applies various data 
    fixing algorithms to clean up the extracted data and can process individual files 
    or entire directories recursively.
    
    Example:
        >>> from transtractor import Parser
        >>> parser = Parser()
    """
    def __init__(self):
        self._inner = LibParser()
    
    def to_dict(self, pdf_file_path: str) -> dict:
        py_text_items = pdf_to_text_items(pdf_file_path)
        return self._inner.py_text_items_to_py_dict(py_text_items)

    def to_debug(self, pdf_file_path: str) -> str:
        py_text_items = pdf_to_text_items(pdf_file_path)
        return self._inner.py_text_items_to_debug_py_str(py_text_items)

    def to_csv(self, pdf_file_path: str, output_file: str) -> None:
        """Convert a bank statement PDF to CSV format and save to output file.

        :param pdf_file_path: Path to the PDF file to be processed
        :param output_file: Path to the output CSV file
        """
        statement_data_dict = self.to_dict(pdf_file_path)
        dict_to_csv(statement_data_dict, output_file)
