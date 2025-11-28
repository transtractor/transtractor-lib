from .transtractor import LibParser
from .structs.statement_data import StatementData
from .utils.write import dict_to_csv
from .utils.extract import pdf_to_text_items


class Parser:
    """A bank statement parser capable of identifying statement types
    and converting them to CSV.
    
    The parser is pre-configured to recognise and extract data 
    from various bank statement PDFs. It can also be extended 
    with custom parsing configurations in cases where a statement format
    is not recognised by the core configurations.

    Example:
        >>> from transtractor import Parser
        >>> parser = Parser()
        >>> statement_dict = parser.to_dict('path/to/statement.pdf')
        >>> parser.to_csv('path/to/statement.pdf', 'output.csv')
        >>> parser.to_debug('path/to/statement.pdf', 'debug_output.txt')
    """
    def __init__(self):
        self._inner = LibParser()

    def parse(self, pdf_file_path: str) -> StatementData:
        """Parse the bank statement PDF and return a StatementData object.

        :param pdf_file_path: Path to the PDF file to be processed
        :return: StatementData object representing the parsed bank statement

        RuntimeError overview (raised indirectly from the Rust core):
        - Unsupported or unidentifiable statement: no matching configuration (typer failure).
        - Quality check failure: no error-free parsed StatementData produced (e.g., unbalanced transactions).
        - Missing required transaction fields: a required date/amount/balance absent.
        """
        py_text_items = pdf_to_text_items(pdf_file_path)
        sd: StatementData = self._inner.py_text_items_to_py_statement_data(py_text_items)
        sd.set_filename(pdf_file_path)
        return sd

    def debug(self, pdf_file_path: str, output_file: str) -> str:
        """Write a summary of the statement data and quality checks for 
        each statement extraction configuration applied.

        :param pdf_file_path: Path to the PDF file to be processed
        :param output_file: Path to the output debug text file
        :return: The debug string written to the output file
        """
        py_text_items = pdf_to_text_items(pdf_file_path)
        result = self._inner.py_text_items_to_debug_py_str(py_text_items)
        with open(output_file, 'w', encoding='utf-8') as fh:
            fh.write(result)
        return result
