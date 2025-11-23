import csv
from datetime import datetime, timezone, timedelta

def dict_to_csv(statement_data_dict: dict, output_file: str) -> None:
    """Write a dictionary representation of bank statement data to a CSV file.

    :param statement_data_dict: Dictionary of bank statement data
    """
    with open(output_file, mode='w', newline='', encoding='utf-8') as file:
        writer = csv.writer(file)
        header = [
            "date",
            "description",
            "amount",
            "balance",
        ]
        writer.writerow(header)

        # Get num rows, assert all columns have same length
        num_rows = None
        for column in header:
            if column in statement_data_dict:
                column_length = len(statement_data_dict[column])
                if num_rows is None:
                    num_rows = column_length
                elif num_rows != column_length:
                    raise ValueError(f"Column '{column}' has inconsistent length.")

        for i in range(num_rows):
            # Convert date from milliseconds since epoch to YYYY-MM-DD format
            date = statement_data_dict["date"][i]
            dt = datetime.fromtimestamp(date / 1000, tz=timezone(timedelta(hours=0)))
            date = dt.strftime("%Y-%m-%d")
            # Leave description
            description = statement_data_dict["description"][i]
            # Round amount and balance to 2 decimal places
            amount = round(float(statement_data_dict["amount"][i]), 2)
            balance = round(float(statement_data_dict["balance"][i]), 2)
            row = [date, description, amount, balance]
            writer.writerow(row)
