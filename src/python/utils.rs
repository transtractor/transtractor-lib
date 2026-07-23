use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyList};

/// Convert a Rust StatementData to a Python StatementData object
pub fn rust_statement_data_to_py_statement_data(
    rust_statement_data: &crate::structs::StatementData,
) -> PyResult<Py<PyAny>> {
    Python::attach(|py| {
        // Import the Python StatementData and Transaction classes
        let statement_data_module = py.import("transtractor.structs.statement_data")?;
        let statement_data_class = statement_data_module.getattr("StatementData")?;

        let transaction_module = py.import("transtractor.structs.transaction")?;
        let transaction_class = transaction_module.getattr("Transaction")?;

        // Get key (required field)
        let key = rust_statement_data.key.as_ref().ok_or_else(|| {
            PyRuntimeError::new_err("StatementData is missing required field: key")
        })?;

        // Get account_number (required field)
        let account_number = rust_statement_data.account_number.as_ref().ok_or_else(|| {
            PyRuntimeError::new_err("StatementData is missing required field: account_number")
        })?;

        // Convert proto_transactions to Transaction objects
        let py_transactions = PyList::empty(py);
        for proto_tx in &rust_statement_data.proto_transactions {
            // Check if the proto transaction is complete
            if !proto_tx.is_ready() {
                return Err(PyRuntimeError::new_err(format!(
                    "Incomplete transaction found: date={:?}, date_index='{}', description='{}', amount={:?}, balance={:?}",
                    proto_tx.date,
                    proto_tx.index,
                    proto_tx.description,
                    proto_tx.amount,
                    proto_tx.balance
                )));
            }

            // Create Python Transaction object
            // Transaction.__init__(date: int, description: str, amount: float, balance: float)
            let py_transaction = transaction_class.call1((
                proto_tx.date.unwrap(),
                proto_tx.index,
                proto_tx.description.clone(),
                proto_tx.amount.unwrap(),
                proto_tx.balance.unwrap(),
            ))?;

            py_transactions.append(py_transaction)?;
        }

        // Create Python StatementData object
        // StatementData(key: str, filename: str, account_number: str, transactions: list[Transaction])
        // filename is set to empty string - to be set by Python calling function
        let py_statement_data =
            statement_data_class.call1((key, account_number, py_transactions))?;

        Ok(py_statement_data.into())
    })
}
