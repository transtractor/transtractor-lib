"""
Type stubs for the transtractor.transtractor module (the Rust extension)
"""

from typing import Any

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
        >>> parser.to_csv("statement.pdf", "output.csv")  # Single file
        >>> parser.test_directory("statements/")         # Batch testing
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
    
    def to_csv(self, input_file: str, output_csv: str) -> None:
        """
        Convert a PDF or TXT bank statement to CSV format.
        
        This method reads a bank statement file (PDF or TXT layout text format), 
        identifies its type using built-in configurations, extracts transaction data, 
        applies data fixing algorithms, and exports the first error-free result to a CSV file.
        
        For PDF files, the method extracts text items using PDF parsing.
        For TXT files, it reads the layout text format and parses into text items.
        The method supports case-insensitive file extensions (.pdf, .PDF, .txt, .TXT).
        
        Args:
            input_file: Path to the input bank statement file (PDF or TXT)
            output_csv: Path where the output CSV file will be written
        
        Raises:
            RuntimeError: If the input file doesn't exist
            RuntimeError: If the file format is unsupported (only .pdf and .txt are supported)
            RuntimeError: If the file cannot be identified as a supported bank statement type
            RuntimeError: If no error-free StatementData is found
            RuntimeError: If there's an error writing the CSV file
        
        Example:
            >>> parser = Parser()
            >>> parser.to_csv("bank_statement.pdf", "transactions.csv")
            >>> parser.to_csv("statement_layout.TXT", "output.csv")  # Case insensitive
        
        Note:
            The output CSV will have columns: date, description, amount, balance
            Dates are formatted as YYYY-MM-DD, amounts are decimal numbers.
            Only the first error-free parsing result is written to CSV.
        """
        ...
    
    def test_directory(self, directory_path: str) -> None:
        """
        Recursively test all PDF and TXT files in a directory and its subdirectories.
        
        This method finds all supported files (PDF and TXT with case-insensitive extensions)
        in the specified directory and all subdirectories, tests each file by attempting to extract
        StatementData, and prints detailed information about the parsing results to stdout.
        
        For each file tested, the method prints:
        - "Reading <file path>..." when starting to test a file
        - "Statement type not supported" if the file is not recognized by any typer
        - Error messages for layout text parsing failures
        
        For each StatementData result, it prints:
        - Statement key (or "Unknown" if not identified)
        - Number of transactions found
        - Time taken to parse in milliseconds
        - Number of errors encountered
        - Status: "PASS" (no errors) or "FAIL" (has errors)
        
        Args:
            directory_path: Path to the directory to test recursively
        
        Raises:
            RuntimeError: If the directory doesn't exist
            RuntimeError: If the path is not a directory
            RuntimeError: If there's an error reading the directory structure
        
        Example:
            >>> parser = Parser()
            >>> parser.test_directory("bank_statements/")
            Reading bank_statements/january/statement1.pdf...
              Key: CBA_Credit_Card, Transactions: 15, Time: 45ms, Errors: 0, Status: PASS
            Reading bank_statements/february/statement2.TXT...
            Statement type not supported
            Reading bank_statements/march/statement3.pdf...
              Key: ING_Debit, Transactions: 23, Time: 62ms, Errors: 2, Status: FAIL
        
        Note:
            This method is designed for batch testing and quality assessment.
            It does not generate CSV files - use to_csv() for individual file conversion.
            The method processes files in directory traversal order (not guaranteed to be sorted).
        """
        ...