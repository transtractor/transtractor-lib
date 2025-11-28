"""Base data structure for recording extracted statement data for 
subsequent processing in Python."""

import csv
from typing import Union

from .transaction import Transaction


def validate_fields(fields: list[str]) -> None:
    """Validate that the provided fields are valid Transaction attributes.
    
    :param fields: List of field names to validate
    :type fields: list[str]
    :raises ValueError: If any field is not a valid 
        Transaction or StatementData attribute
    """
    valid_fields = {'date', 'date_index', 'description', 'amount', 'balance',
                    'key', 'filename', 'account_number'}
    for field in fields:
        if field not in valid_fields:
            raise ValueError(f"Invalid field: {field}. Valid fields are: {valid_fields}")


class StatementData:
    """Class representing bank statement data."""

    def __init__(self, key: str, account_number: str, transactions: list[Transaction]):
        """Initialize StatementData with validated attributes.

        :param key: Unique identifier for the statement
        :type key: str
        :param account_number: Account number associated with the statement
        :type account_number: str
        :param transactions: List of transactions in the statement
        :type transactions: list[Transaction]
        """
        self._key = None
        self._filename = ""
        self._account_number = None
        self._transactions = []

        # Use setters to enforce types
        self.set_key(key)
        self.set_account_number(account_number)
        self.set_transactions(transactions)

    def __repr__(self) -> str:
        return (f"StatementData(key={self._key!r}, "
                f"filename={self._filename!r}, "
                f"account_number={self._account_number!r}, "
                f"transactions=[{len(self._transactions)} transactions])")

    @property
    def key(self) -> str:
        """Get the statement key."""
        return self._key

    @property
    def filename(self) -> str:
        """Get the filename."""
        return self._filename

    @property
    def account_number(self) -> str:
        """Get the account number."""
        return self._account_number

    @property
    def transactions(self) -> list[Transaction]:
        """Get the list of transactions."""
        return self._transactions

    def set_key(self, key: str) -> None:
        """Set the key for the statement data.

        :param key: Unique identifier for the statement
        :type key: str
        :raises TypeError: If key is not a string
        """
        if not isinstance(key, str):
            raise TypeError(f"key must be a string, got {type(key).__name__}")
        self._key = key

    def set_filename(self, filename: str) -> None:
        """Set the filename for the statement data.

        :param filename: Filename for the statement
        :type filename: str
        :raises TypeError: If filename is not a string
        """
        if not isinstance(filename, str):
            raise TypeError(f"filename must be a string, got {type(filename).__name__}")
        self._filename = filename

    def set_account_number(self, account_number: str) -> None:
        """Set the account number for the statement data.

        :param account_number: Account number associated with the statement
        :type account_number: str
        :raises TypeError: If account_number is not a string
        """
        if not isinstance(account_number, str):
            raise TypeError(f"account_number must be a string, got {type(account_number).__name__}")
        self._account_number = account_number

    def set_transactions(self, transactions: list[Transaction]) -> None:
        """Set the transactions for the statement data.

        :param transactions: List of transactions
        :type transactions: list[Transaction]
        :raises TypeError: If transactions is not a list or contains non-Transaction items
        """
        if not isinstance(transactions, list):
            raise TypeError(f"transactions must be a list, got {type(transactions).__name__}")

        for i, transaction in enumerate(transactions):
            if not isinstance(transaction, Transaction):
                raise TypeError(
                    f"transactions[{i}] must be a Transaction instance, "
                    f"got {type(transaction).__name__}"
                )

        self._transactions = transactions

    def to_csv(self,
            file_path: str,
            fields: Union[tuple[str, ...], list[str]] = ('date', 'description', 'amount', 'balance')
        ) -> None:
        """Export the statement data to a CSV file.

        :param file_path: Path to the output CSV file
        :type file_path: str
        :param fields: Fields to include in the CSV. Defaults to
            ('date', 'description', 'amount', 'balance'). Valid fields are:
            'date', 'date_index', 'description', 'amount', 'balance',
            'key', 'filename', 'account_number'.
        :type fields: Union[tuple[str, ...], list[str]]

        Example usage::

            # Export with default fields
            statement_data.to_csv('transactions.csv')

            # Export with all available fields using list (or tuple)
            statement_data.to_csv(
                'full_export.csv',
                fields=['date', 'date_index', 'description', 'amount',
                        'balance', 'key', 'filename', 'account_number']
            )
        """
        # Validate fields
        validate_fields(list(fields))

        with open(file_path, mode='w', newline='', encoding='utf-8') as csvfile:
            writer = csv.writer(csvfile)
            # Write header
            writer.writerow(fields)
            # Write transaction data
            for transaction in self._transactions:
                row = []
                for field in fields:
                    if field in {'key', 'filename', 'account_number'}:
                        value = getattr(self, f"_{field}", None)
                    else:
                        value = getattr(transaction, field, None)
                    row.append(value)
                writer.writerow(row)

    def to_pandas_dict(self,
            fields: Union[tuple[str, ...], list[str]] = ('date', 'description', 'amount', 'balance')
        ) -> dict[str, list]:
        """Convert the statement data to a dictionary suitable for pandas DataFrame.

        :param fields: Fields to include in the dictionary. Defaults to
            ('date', 'description', 'amount', 'balance').
        :return: Dictionary with keys as field names and values as lists of field values
        :rtype: dict[str, list]

        Example usage::

            # Default fields
            data_dict = statement_data.to_pandas_dict()
            df = pd.DataFrame(data_dict)

            # Custom fields with list (tuple also supported)
            data_dict = statement_data.to_pandas_dict(
                fields=['date', 'description', 'amount', 'balance', 'key']
            )
            df = pd.DataFrame(data_dict)
        """
        validate_fields(list(fields))
        data_dict = {field: [] for field in fields}

        for transaction in self._transactions:
            for field in fields:
                if field in {'key', 'filename', 'account_number'}:
                    value = getattr(self, f"_{field}", None)
                else:
                    value = getattr(transaction, field, None)
                data_dict[field].append(value)

        return data_dict
