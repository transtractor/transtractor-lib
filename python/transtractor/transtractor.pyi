"""
Type stubs for the transtractor.transtractor module (the Rust extension)
"""

from typing import Any

class Parser:
    """
    A PDF statement parser that can identify bank statement types and convert them to CSV format.
    
    The parser uses predefined configurations to identify different bank statement formats
    and applies various data fixing algorithms to clean up the extracted data.
    
    Example:
        >>> from transtractor import Parser
        >>> parser = Parser()
        >>> parser.pdf_to_csv("statement.pdf", "output.csv")
    """
    
    def __init__(self) -> None:
        """
        Create a new Parser instance.
        
        Returns:
            A new parser instance ready to process PDF statements.
        
        Example:
            >>> parser = Parser()
        """
        ...
    
    def pdf_to_csv(self, input_pdf: str, output_csv: str) -> None:
        """
        Convert a PDF bank statement to CSV format.
        
        This method reads a PDF bank statement, identifies its type using built-in
        configurations, extracts transaction data, applies data fixing algorithms,
        and exports the results to a CSV file.
        
        Args:
            input_pdf: Path to the input PDF bank statement file
            output_csv: Path where the output CSV file will be written
        
        Raises:
            RuntimeError: If the input PDF file doesn't exist
            RuntimeError: If the PDF cannot be identified as a supported bank statement type
            RuntimeError: If there's an error writing the CSV file
        
        Example:
            >>> parser = Parser()
            >>> parser.pdf_to_csv("bank_statement.pdf", "transactions.csv")
        
        Note:
            The output CSV will have columns: date, description, amount, balance
            Dates are formatted as YYYY-MM-DD, amounts are decimal numbers.
        """
        ...