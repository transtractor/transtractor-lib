"""Python implementation of transaction data for further processing in Python."""

from dataclasses import dataclass
from datetime import date as Date
from datetime import datetime
from typing import Union


@dataclass
class Transaction:
    """Class representing a bank transaction."""
    date: Date
    date_index: int
    description: str
    amount: float
    balance: float

    def __init__(self,
            date: Union[Date, int],
            date_index: int,
            description:str,
            amount: float,
            balance: float
        ):
        # pylint: disable=too-many-arguments,too-many-positional-arguments
        """Initialize a Transaction.
        
        :param date: Either a date object or milliseconds since epoch (int)
        :param date_index: Transaction index for the day
        :param description: Transaction description
        :param amount: Transaction amount (will be rounded to 2 decimal places)
        :param balance: Account balance (will be rounded to 2 decimal places)
        """
        if isinstance(date, int):
            # Convert milliseconds since epoch to date
            self.date = datetime.fromtimestamp(date / 1000.0).date()
        else:
            self.date = date
        self.date_index = date_index
        self.description = description
        self.amount = round(amount, 2)
        self.balance = round(balance, 2)
