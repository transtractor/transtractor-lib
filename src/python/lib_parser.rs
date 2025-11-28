use crate::configs::StatementTyper;
use crate::parsers::flows::text_items_to_debug::text_items_to_debug;
use crate::parsers::flows::text_items_to_statement_datas::text_items_to_statement_datas;
use crate::python::utils;
use pyo3::prelude::*;

#[pyclass]
pub struct LibParser {
    typer: StatementTyper,
}

#[pymethods]
impl LibParser {
    /// Create a new Parser instance
    #[new]
    pub fn new() -> Self {
        Self {
            typer: StatementTyper::new(),
        }
    }

    /// Process a Python list of text items and return statement data as a
    /// Python object of type StatementData.
    pub fn py_text_items_to_py_statement_data(
        &self,
        py_text_items: &Bound<'_, pyo3::types::PyAny>,
    ) -> PyResult<PyObject> {
        let mut text_items = utils::py_text_items_to_rust_text_items(py_text_items)?;
            let statement_data_results = text_items_to_statement_datas(&mut text_items, &self.typer)
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e))?;
            // Find the first error-free StatementData
            for data in statement_data_results {
                if data.errors.is_empty() {
                    return utils::rust_statement_data_to_py_statement_data(&data);
                }
            }
            Err(pyo3::exceptions::PyRuntimeError::new_err("No error-free StatementData found"))
    }

    /// Process a Python list of text items and return debug information as a string.
    pub fn py_text_items_to_debug_py_str(
        &self,
        py_text_items: &Bound<'_, pyo3::types::PyAny>,
    ) -> PyResult<String> {
        let mut text_items = utils::py_text_items_to_rust_text_items(py_text_items)?;
        match text_items_to_debug(&mut text_items, &self.typer) {
            Ok(debug_str) => Ok(debug_str),
            Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(e)),
        }
    }
}
