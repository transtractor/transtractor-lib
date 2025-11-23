from .transtractor import LibParser
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

    def to_dict(self, pdf_file_path: str) -> dict[str, list]:
        """Extract bank statement into a dictionary suitable for reading into 
        a DataFrame.

        :param pdf_file_path: Path to the PDF file to be processed
        :return: Dictionary of lists representation of the bank statement data. 
            Format is:
            {
                "date": [int],  # milliseconds since epoch
                "transaction_index": [int], # Transaction index for the day
                "description": [str], # Description of the transaction
                "amount": [float], # Amount of the transaction
                "balance": [float], # Balance after the transaction
            }

        RuntimeError overview (raised indirectly from the Rust core):
        - File access failure: PDF cannot be opened or read.
        - Unsupported or unidentifiable statement: no matching configuration (typer failure).
        - Quality check failure: no error-free parsed StatementData produced (e.g., unbalanced transactions).
        - Missing required transaction fields: a required date/amount/balance absent.
        - Parsing configuration error: internal parsing logic signals inconsistent data state.
        - Invalid text items: structural issues converting extracted items before parsing.
        The error message string will describe the specific failure cause.
        """        
        py_text_items = pdf_to_text_items(pdf_file_path)
        return self._inner.py_text_items_to_py_dict(py_text_items)

    def to_debug(self, pdf_file_path: str, output_file: str) -> str:
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

    def to_csv(self, pdf_file_path: str, output_file: str) -> None:
        """Convert a bank statement PDF to CSV format and save to output file.

        :param pdf_file_path: Path to the PDF file to be processed
        :param output_file: Path to the output CSV file

        RuntimeError overview (raised indirectly from the Rust core):
        - File access failure: PDF cannot be opened or read.
        - Unsupported or unidentifiable statement: no matching configuration (typer failure).
        - Quality check failure: no error-free parsed StatementData produced (e.g., unbalanced transactions).
        - Missing required transaction fields: a required date/amount/balance absent.
        - Parsing configuration error: internal parsing logic signals inconsistent data state.
        - Invalid text items: structural issues converting extracted items before parsing.
        The error message string will describe the specific failure cause.
        """
        statement_data_dict = self.to_dict(pdf_file_path)
        dict_to_csv(statement_data_dict, output_file)
