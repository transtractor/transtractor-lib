import csv


def dict_to_csv(statement_data_dict: dict, output_file: str) -> None:
    """Write a dictionary representation of bank statement data to a CSV file.

    :param statement_data_dict: Dictionary of bank statement data
    """
    with open(output_file, mode='w', newline='', encoding='utf-8') as file:
        writer = csv.writer(file)
        header = [
            "date",
            "transaction_index",
            "description",
            "amount",
            "balance",
        ]

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
            row = [statement_data_dict.get(col, [""] * num_rows)[i] for col in header]
            writer.writerow(row)
